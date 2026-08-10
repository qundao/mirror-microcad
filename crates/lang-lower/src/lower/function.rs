// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    CastInto, Lower, LowerContext, LowerError, LowerResult, ir,
    lower::{LowerName, extract_statements_with_tail, for_each_statement},
};

use microcad_builtin::__mu;
use microcad_lang_base::{Identifier, SpanToSrcRef, SrcRef, SrcReferrer};
use microcad_lang_parse::ast;

impl Lower<ast::def::Function> for ir::OuterAttributes {
    fn lower(node: &ast::def::Function, context: &mut LowerContext) -> LowerResult<Self> {
        crate::lower::attribute::outer_with_doc(&node.doc, &node.attr, context)
    }
}

impl Lower<ast::def::Function> for ir::FunctionSignature {
    fn lower(node: &ast::def::Function, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.span_to_src_ref(&node.span),
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            return_type: match &node.return_type {
                Some(ty) => Some(ir::Type::lower(ty, context)?),
                None => None,
            },
        })
    }
}

impl<Name: LowerName> Lower<ast::Body> for ir::Scope<Name> {
    fn lower(node: &ast::Body, context: &mut LowerContext) -> LowerResult<Self> {
        let statements = &node.statements;
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.span_to_src_ref(&stmt.span());
            use ast::Statement::*;
            match stmt {
                FileModule(_) | Const(_) | Use(_) | InlineModule(_) | Init(_) | Workbench(_)
                | Function(_) | Property(_) | InnerAttribute(_) | InnerDocComment(_) | Error(_) => {
                    context.diag(LowerError::StatementNotAllowed { src_ref });
                }
                _ => {}
            }
            Ok(())
        })?;

        let statements: Box<[ir::FunctionStatement<Name>]> = Box::lower(statements, context)?;

        Ok(Self {
            statements,
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl<Name: LowerName> Lower<ast::Expression> for ir::FunctionExpression<Name> {
    fn lower(node: &ast::Expression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Expression::Call(expr) => Self::Call(ir::Call::lower(expr, context)?),
            ast::Expression::Bracketed(expr, _) => Self::lower(expr.as_ref(), context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                ..
            }) => Self::Literal(ir::Literal::from_value(s.content.clone())),
            ast::Expression::Literal(expr) => Self::Literal(ir::Literal::lower(expr, context)?),
            ast::Expression::String(s) => Self::Call(ir::Call::lower(s, context)?.cast_into()),
            ast::Expression::Tuple(t) => Self::Call(ir::Call::lower(t, context)?),
            ast::Expression::ArrayRange(a) => Self::lower(a, context)?,
            ast::Expression::ArrayList(a) => Self::lower(a, context)?,
            ast::Expression::SymbolPath(n) => Self::Name(Name::lower(n, context)?),
            ast::Expression::BinaryOperation(binop) => Self::Call(ir::Call::lower(binop, context)?),
            ast::Expression::UnaryOperation(unop) => Self::Call(ir::Call::lower(unop, context)?),
            ast::Expression::Marker(_) => {
                panic!("Marker statement not allowed")
            }
            ast::Expression::Body(body) => Self::Scope(ir::Scope::lower(body, context)?),
            ast::Expression::ElementAccess(access) => access.element_chain.iter().try_fold(
                Self::lower(access.expr.as_ref(), context)?,
                |lhs, element| -> LowerResult<Self> {
                    use ast::ElementInner::*;
                    let src_ref = context.span_to_src_ref(&access.span);

                    Ok(match &element.inner {
                        Attribute(_) => panic!("Attribute access not allowed"),
                        Tuple(t) => Self::Call(ir::Call {
                            name: __mu!(core::member_access),
                            args: ir::ArgumentList::from_iter([
                                lhs,
                                Self::Name(Name::from(t.name.to_string())),
                            ]),
                            src_ref,
                        }),
                        Method(m) => Self::Call(ir::Call {
                            name: Name::lower(&m.name, context)?,
                            args: ir::ArgumentList::lower(&m.arguments, context)?.prepended(lhs),
                            src_ref,
                        }),
                        ArrayElement(e) => Self::Call(ir::Call {
                            name: __mu!(core::array_access),
                            args: ir::ArgumentList::from_iter([
                                lhs,
                                Self::lower(e.as_ref(), context)?,
                            ]),
                            src_ref,
                        }),
                    })
                },
            )?,
            ast::Expression::If(if_expr) => Self::If(ir::If::lower(if_expr, context)?),
            ast::Expression::Error(_) => todo!(),
        })
    }
}

impl<Name: LowerName> Lower<Option<ast::Expression>> for Option<ir::FunctionExpression<Name>> {
    fn lower(node: &Option<ast::Expression>, context: &mut LowerContext) -> LowerResult<Self> {
        node.as_ref()
            .map(|expr| ir::FunctionExpression::lower(expr, context))
            .transpose()
    }
}

impl<Name: LowerName> Lower<ast::Return> for ir::ReturnStatement<Name> {
    fn lower(node: &ast::Return, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            expr: Option::<ir::FunctionExpression<Name>>::lower(&node.expr, context)?,
            keyword_src_ref: context.span_to_src_ref(&node.keyword_span),
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl<Name: LowerName> Lower<ast::Statement> for Option<ir::FunctionStatement<Name>> {
    fn lower(stmt: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match stmt {
            ast::Statement::Return(ret) => Some(ir::FunctionStatement::Return(
                ir::ReturnStatement::lower(ret, context)?,
            )),
            ast::Statement::LocalAssignment(local_assignment) => {
                Some(ir::FunctionStatement::Local(ir::LocalAssignment::<
                    ir::FunctionExpression<Name>,
                >::lower(
                    local_assignment, context
                )?))
            }
            ast::Statement::Expression(expression_statement) => {
                let src_ref = context.span_to_src_ref(&expression_statement.span);
                use ast::Expression::*;
                match &expression_statement.expr {
                    Literal(_)
                    | Bracketed(_, _)
                    | Tuple(_)
                    | ArrayRange(_)
                    | ArrayList(_)
                    | String(_)
                    | SymbolPath(_)
                    | BinaryOperation(_)
                    | UnaryOperation(_)
                    | ElementAccess(_) => {
                        context.diag(LowerError::FunctionStatementIgnored(src_ref));
                        None
                    }
                    ast::Expression::Marker(_) | ast::Expression::Error(_) => {
                        context.diag(LowerError::StatementNotAllowed { src_ref });
                        None
                    }
                    ast::Expression::Call(call) => Some(ir::Call::lower(call, context)?.into()),
                    ast::Expression::Body(body) => Some(ir::Scope::lower(body, context)?.into()),
                    ast::Expression::If(if_) => Some(ir::If::lower(if_, context)?.into()),
                }
            }
            _ => None,
        })
    }
}

impl<Name: LowerName> Lower<ast::StatementList> for Box<[ir::FunctionStatement<Name>]> {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        let statements = extract_statements_with_tail(
            node,
            context,
            Option::<ir::FunctionStatement<Name>>::lower,
            // Lower Tail expression to Return statements.
            |tail, context| {
                Ok(Some(ir::FunctionStatement::Tail(
                    ir::FunctionExpression::lower(&tail.expr, context)?,
                )))
            },
        )?;

        let mut return_src_ref = SrcRef::none();

        for stmt in statements.iter() {
            let src_ref = stmt.src_ref();
            if return_src_ref.is_some() {
                // We've already hit a return, so everything after it is unreachable dead code.
                context.diag(LowerError::Unreachable {
                    src_ref,
                    last_ref: return_src_ref,
                });
            } else if let ir::FunctionStatement::Return(ret) = stmt {
                // Found the first return statement!
                return_src_ref = ret.src_ref;
            }
        }

        Ok(statements)
    }
}

impl Lower<ast::StatementList> for ir::FunctionItems {
    fn lower(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.span_to_src_ref(&stmt.span());
            use ast::Statement::*;
            match stmt {
                Init(_) | Workbench(_) | InlineModule(_) | FileModule(_) | Property(_)
                | Error(_) => context.diag(LowerError::StatementNotAllowed { src_ref }),
                _ => {}
            }
            Ok(())
        })?;

        Ok(Self {
            aliases: ir::Aliases::lower(statements, context)?,
            constants: Box::lower(statements, context)?,
        })
    }
}

impl Lower<ast::def::Function> for ir::Function {
    fn lower(node: &ast::def::Function, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.span_to_src_ref(&node.span),
            outer_attr: ir::OuterAttributes::lower(node, context)?,
            visibility: ir::Visibility::lower(&node.vis, context)?,
            keyword_ref: context.span_to_src_ref(&node.keyword_span),
            id: Identifier::lower(&node.id, context)?,
            signature: ir::FunctionSignature::lower(node, context)?,
            inner_attr: ir::InnerAttributes::lower(&node.body.statements, context)?,
            items: ir::FunctionItems::lower(&node.body.statements, context)?,
            statements: Box::lower(&node.body.statements, context)?,
        })
    }
}

impl Lower<ast::Statement> for Option<ir::Function> {
    fn lower(node: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Statement::Function(f) => Some(ir::Function::lower(f, context)?),
            _ => None,
        })
    }
}

impl Lower<ast::ExpressionStatement> for Option<ir::Function> {
    fn lower(_: &ast::ExpressionStatement, _: &mut LowerContext) -> LowerResult<Self> {
        Ok(None)
    }
}
