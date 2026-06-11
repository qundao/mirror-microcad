// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Lower, LowerContext, LowerResult,
    ir::{self, FunctionExpression},
    lower::{extract_statements, extract_statements_with_tail},
};

use microcad_lang_base::{Identifier, Refer, SrcRef};
use microcad_lang_parse::ast;

impl Lower<ast::FunctionDefinition> for ir::Attributes {
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

impl Lower<ast::Body> for ir::Scope {
    fn lower(node: &ast::Body, context: &mut LowerContext) -> LowerResult<Self> {
        // TODO: Check for not-allowed statements here
        let statements = &node.statements;

        Ok(Self(Refer::new(
            ir::FunctionStatements::lower(statements, context)?,
            context.src_ref(&node.span),
        )))
    }
}

impl Lower<ast::Expression> for ir::FunctionExpression {
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
            ast::Expression::QualifiedName(n) => Self::Name(ir::QualifiedName::lower(n, context)?),
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

impl Lower<Option<ast::Expression>> for Option<ir::FunctionExpression> {
    fn lower(node: &Option<ast::Expression>, context: &mut LowerContext) -> LowerResult<Self> {
        node.as_ref()
            .map(|expr| ir::FunctionExpression::lower(expr, context))
            .transpose()
    }
}

impl Lower<ast::Return> for ir::ReturnStatement {
    fn lower(node: &ast::Return, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            value: Option::<ir::FunctionExpression>::lower(&node.value, context)?,
            keyword_src_ref: context.src_ref(&node.keyword_span),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower<ast::Statement> for Option<ir::FunctionStatement> {
    fn lower(stmt: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match stmt {
            ast::Statement::Return(ret) => Some(ir::FunctionStatement::Return(
                ir::ReturnStatement::lower(ret, context)?,
            )),
            ast::Statement::LocalAssignment(local_assignment) => {
                Some(ir::FunctionStatement::Local(ir::LocalAssignment::<
                    FunctionExpression,
                >::lower(
                    local_assignment, context
                )?))
            }
            ast::Statement::Expression(expression_statement) => {
                Some(ir::FunctionStatement::Expression(
                    ir::FunctionExpression::lower(&expression_statement.expression, context)?,
                ))
            }
            _ => None,
        })
    }
}

impl Lower<ast::StatementList> for ir::FunctionStatements {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements_with_tail(
            node,
            context,
            |stmt, context| Option::<ir::FunctionStatement>::lower(stmt, context),
            // Lower Tail expression to Return statements.
            |tail, context| {
                Ok(ir::FunctionStatement::Return(ir::ReturnStatement {
                    value: Some(ir::FunctionExpression::lower(&tail.expression, context)?),
                    keyword_src_ref: SrcRef::none(),
                    src_ref: context.src_ref(&tail.span),
                }))
            },
        )?))
    }
}

impl Lower<ast::FunctionDefinition> for ir::Function {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            outer_attr: ir::Attributes::lower(node, context)?,
            visibility: ir::Visibility::lower(&node.visibility, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            id: Identifier::lower(&node.name, context)?,
            signature: ir::FunctionSignature::lower(&node, context)?,
            inner_attr: ir::Attributes::lower(&node.body.statements, context)?,
            aliases: ir::Aliases::lower(&node.body.statements, context)?,
            constants: ir::Constants::lower(&node.body.statements, context)?,
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
