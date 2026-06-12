// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Lower, LowerContext, LowerError, LowerResult,
    ir::{self, FunctionExpression},
    lower::{extract_statements, extract_statements_with_tail, for_each_statement},
};

use microcad_lang_base::{Identifier, PushDiag, Refer, SrcRef, SrcReferrer};
use microcad_lang_parse::ast;
use serde::Serialize;

impl Lower<ast::FunctionDefinition> for ir::OuterAttributes {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        crate::lower::attribute::outer_with_doc(&node.doc, &node.attributes, context)
    }
}

impl Lower<ast::FunctionDefinition> for ir::FunctionSignature {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            return_type: Option::<ir::TypeAnnotation>::lower(&node.return_type, context)?,
        })
    }
}

impl Lower<ast::FunctionDefinition> for ir::Constants {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements(&node.body.statements, |stmt| {
            Ok(match stmt {
                ast::Statement::Const(const_assignment) => {
                    Some(ir::Constant::lower(&const_assignment, context)?)
                }
                _ => None,
            })
        })?))
    }
}

impl<NAME: Serialize> Lower<ast::Body> for ir::Scope<NAME>
where
    NAME: SrcReferrer + Lower<ast::QualifiedName>,
{
    fn lower(node: &ast::Body, context: &mut LowerContext) -> LowerResult<Self> {
        let statements = &node.statements;
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.src_ref(&stmt.span());
            use ast::Statement::*;
            Ok(match stmt {
                FileModule(_) | Const(_) | Use(_) | InlineModule(_) | Init(_) | Workbench(_)
                | Function(_) | Property(_) | InnerAttribute(_) | InnerDocComment(_) | Error(_) => {
                    context
                        .diagnostics
                        .error(&src_ref, LowerError::StatementNotAllowed { src_ref })?
                }
                _ => {}
            })
        })?;

        Ok(Self(Refer::new(
            ir::FunctionStatements::lower(statements, context)?,
            context.src_ref(&node.span),
        )))
    }
}

impl<NAME: Serialize> Lower<ast::Expression> for ir::FunctionExpression<NAME>
where
    NAME: SrcReferrer + Lower<ast::QualifiedName>,
{
    fn lower(node: &ast::Expression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Expression::Call(expr) => Self::Call(ir::Call::lower(expr, context)?),
            ast::Expression::Bracketed(expr, _) => Self::lower(expr, context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                ..
            }) => Self::FormatString(ir::FormatString::lower(s, context)?),
            ast::Expression::Literal(expr) => Self::Literal(ir::Literal::lower(expr, context)?),
            ast::Expression::String(s) => Self::FormatString(ir::FormatString::lower(s, context)?),
            ast::Expression::Tuple(t) => {
                Self::TupleExpression(ir::TupleExpression::lower(t, context)?)
            }
            ast::Expression::ArrayRange(a) => Self::ArrayExpression(ir::ArrayExpression {
                inner: ir::ArrayExpressionInner::Range(ir::RangeExpression::lower(a, context)?),
                unit: ir::Unit::lower(&a.unit, context)?,
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::ArrayList(a) => Self::ArrayExpression(ir::ArrayExpression {
                inner: ir::ArrayExpressionInner::List(ir::ListExpression::lower(a, context)?),
                unit: ir::Unit::lower(&a.unit, context)?,
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::QualifiedName(n) => Self::Name(NAME::lower(n, context)?),
            ast::Expression::BinaryOperation(binop) => {
                Self::BinaryOp(ir::BinaryOp::lower(binop, context)?)
            }
            ast::Expression::UnaryOperation(unop) => {
                Self::UnaryOp(ir::UnaryOp::lower(unop, context)?)
            }
            ast::Expression::Marker(_) => {
                panic!("Marker statement not allowed")
            }
            ast::Expression::Body(body) => Self::Scope(ir::Scope::lower(body, context)?),
            ast::Expression::ElementAccess(access) => access.element_chain.iter().try_fold(
                Self::lower(&access.value, context)?,
                |acc, element| -> LowerResult<Self> {
                    use ast::ElementInner::*;
                    let src_ref = context.src_ref(&access.span);
                    let lhs = Box::new(acc);

                    Ok(match &element.inner {
                        Attribute(_) => panic!("Attribute access not allowed"),
                        Tuple(t) => Self::TupleAccess(ir::ElementAccess {
                            lhs,
                            element: ir::Identifier::lower(t, context)?,
                            src_ref,
                        }),
                        Method(m) => Self::MethodCall(ir::ElementAccess {
                            lhs,
                            element: ir::Call::lower(m, context)?,
                            src_ref,
                        }),
                        ArrayElement(e) => Self::ArrayAccess(ir::ElementAccess {
                            lhs,
                            element: Box::new(ir::FunctionExpression::lower(e, context)?),
                            src_ref,
                        }),
                    })
                },
            )?,
            ast::Expression::If(if_expr) => Self::If(ir::If::lower(&if_expr, context)?),
            ast::Expression::Error(_) => todo!(),
        })
    }
}

impl<NAME: Serialize> Lower<Option<ast::Expression>> for Option<ir::FunctionExpression<NAME>>
where
    NAME: SrcReferrer + Lower<ast::QualifiedName>,
{
    fn lower(node: &Option<ast::Expression>, context: &mut LowerContext) -> LowerResult<Self> {
        node.as_ref()
            .map(|expr| ir::FunctionExpression::lower(expr, context))
            .transpose()
    }
}

impl<NAME: Serialize> Lower<ast::Return> for ir::ReturnStatement<NAME>
where
    NAME: SrcReferrer + Lower<ast::QualifiedName>,
{
    fn lower(node: &ast::Return, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            value: Option::<ir::FunctionExpression<NAME>>::lower(&node.value, context)?,
            keyword_src_ref: context.src_ref(&node.keyword_span),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl<NAME: Serialize> Lower<ast::Statement> for Option<ir::FunctionStatement<NAME>>
where
    NAME: SrcReferrer + Lower<ast::QualifiedName>,
{
    fn lower(stmt: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match stmt {
            ast::Statement::Return(ret) => Some(ir::FunctionStatement::Return(
                ir::ReturnStatement::lower(ret, context)?,
            )),
            ast::Statement::LocalAssignment(local_assignment) => {
                Some(ir::FunctionStatement::Local(ir::LocalAssignment::<
                    FunctionExpression<NAME>,
                >::lower(
                    local_assignment, context
                )?))
            }
            ast::Statement::Expression(expression_statement) => {
                Some(ir::FunctionStatement::Expression(ir::FunctionExpression::<
                    NAME,
                >::lower(
                    &expression_statement.expression,
                    context,
                )?))
            }
            _ => None,
        })
    }
}

impl<NAME> Lower<ast::StatementList> for ir::FunctionStatements<NAME>
where
    NAME: SrcReferrer + Serialize + Lower<ast::QualifiedName>,
{
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        let statements = extract_statements_with_tail(
            node,
            context,
            |stmt, context| Option::<ir::FunctionStatement<NAME>>::lower(stmt, context),
            // Lower Tail expression to Return statements.
            |tail, context| {
                Ok(ir::FunctionStatement::Return(ir::ReturnStatement {
                    value: Some(ir::FunctionExpression::lower(&tail.expression, context)?),
                    keyword_src_ref: SrcRef::none(),
                    src_ref: context.src_ref(&tail.span),
                }))
            },
        )?;

        let mut return_src_ref = SrcRef::none();

        for stmt in statements.iter() {
            let src_ref = stmt.src_ref();
            if return_src_ref.is_some() {
                // We've already hit a return, so everything after it is unreachable dead code.
                context.diagnostics.warning(
                    &src_ref,
                    LowerError::Unreachable {
                        src_ref,
                        last_ref: return_src_ref,
                    },
                )?;
            } else if let ir::FunctionStatement::Return(ret) = stmt {
                // Found the first return statement!
                return_src_ref = ret.src_ref;
            }
        }

        Ok(Self(statements))
    }
}

impl Lower<ast::StatementList> for ir::FunctionItems {
    fn lower(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.src_ref(&stmt.span());
            use ast::Statement::*;
            Ok(match stmt {
                Init(_) | Workbench(_) | InlineModule(_) | FileModule(_) | Property(_)
                | Error(_) => context
                    .diagnostics
                    .error(&src_ref, LowerError::StatementNotAllowed { src_ref })?,
                _ => {}
            })
        })?;

        Ok(Self {
            aliases: ir::Aliases::lower(statements, context)?,
            constants: ir::Constants::lower(statements, context)?,
        })
    }
}

impl Lower<ast::FunctionDefinition> for ir::Function {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            outer_attr: ir::OuterAttributes::lower(node, context)?,
            visibility: ir::Visibility::lower(&node.visibility, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            id: Identifier::lower(&node.name, context)?,
            signature: ir::FunctionSignature::lower(&node, context)?,
            inner_attr: ir::InnerAttributes::lower(&node.body.statements, context)?,
            items: ir::FunctionItems::lower(&node.body.statements, context)?,
            statements: ir::FunctionStatements::lower(&node.body.statements, context)?,
        })
    }
}

impl Lower<ast::StatementList> for ir::Functions {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::Function(function_definition) => {
                    Some(ir::Function::lower(function_definition, context)?)
                }
                _ => None,
            })
        })?))
    }
}
