// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, LowerResult, ir};

mod call;
mod literal;

use microcad_lang_base::{Identifier, PushDiag, Refer};
use microcad_lang_parse::ast;

impl<EXPR> Lower<ast::BinaryOperation> for ir::BinaryOp<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::BinaryOperation, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            lhs: Box::new(EXPR::lower(node.lhs.as_ref(), context)?),
            rhs: Box::new(EXPR::lower(node.rhs.as_ref(), context)?),
            op: Refer::new(
                node.operation.operation.as_str().into(),
                context.src_ref(&node.operation.span),
            ),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl<EXPR> Lower<ast::UnaryOperation> for ir::UnaryOp<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::UnaryOperation, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(ir::UnaryOp {
            rhs: Box::new(EXPR::lower(&node.rhs, context)?),
            op: Refer::new(
                node.operation.operation.as_str().into(),
                context.src_ref(&node.operation.span),
            ),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl<EXPR> Lower<ast::ArrayItem> for ir::RangeFirst<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::ArrayItem, context: &mut LowerContext) -> LowerResult<Self> {
        if matches!(
            node.expression,
            ast::Expression::Literal(
                ast::Literal {
                    literal: ast::LiteralKind::Float(_)
                        | ast::LiteralKind::String(_)
                        | ast::LiteralKind::Quantity(_)
                        | ast::LiteralKind::Bool(_),
                    ..
                },
                ..
            )
        ) {
            return Err(LowerError::InvalidRangeType {
                src_ref: context.src_ref(&node.expression.span()),
            });
        }
        Ok(ir::RangeFirst(Box::new(EXPR::lower(
            &node.expression,
            context,
        )?)))
    }
}

impl<EXPR> Lower<ast::ArrayItem> for ir::RangeLast<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::ArrayItem, context: &mut LowerContext) -> LowerResult<Self> {
        if matches!(
            node.expression,
            ast::Expression::Literal(
                ast::Literal {
                    literal: ast::LiteralKind::Float(_)
                        | ast::LiteralKind::String(_)
                        | ast::LiteralKind::Quantity(_)
                        | ast::LiteralKind::Bool(_),
                    ..
                },
                ..
            )
        ) {
            return Err(LowerError::InvalidRangeType {
                src_ref: context.src_ref(&node.expression.span()),
            });
        }
        Ok(ir::RangeLast(Box::new(EXPR::lower(
            &node.expression,
            context,
        )?)))
    }
}

impl<EXPR> Lower<ast::ArrayRangeExpression> for ir::RangeExpression<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::ArrayRangeExpression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            first: ir::RangeFirst::lower(&node.start, context)?,
            last: ir::RangeLast::lower(&node.end, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl<EXPR> Lower<ast::ArrayListExpression> for ir::ListExpression<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::ArrayListExpression, context: &mut LowerContext) -> LowerResult<Self> {
        node.items
            .iter()
            .map(|item| EXPR::lower(&item.expression, context))
            .collect::<Result<Vec<EXPR>, _>>()
    }
}

impl Lower<ast::Identifier> for ir::Marker {
    fn lower(node: &ast::Identifier, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            id: Identifier::lower(node, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

/*
impl Lower for ir::Expression {
    type AstNode = ast::Expression;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Expression::Call(expr) => ir::Expression::Call(ir::Call::lower(expr, context)?),
            ast::Expression::Bracketed(expr, _) => ir::Expression::lower(expr, context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                span,
                ..
            }) => ir::Expression::FormatString(ir::FormatString(Refer::new(
                vec![ir::FormatStringInner::String(Refer::new(
                    s.content.clone(),
                    context.src_ref(&s.span),
                ))],
                context.src_ref(span),
            ))),
            ast::Expression::Literal(expr) => {
                ir::Expression::Literal(ir::Literal::lower(expr, context)?)
            }
            ast::Expression::String(s) => {
                ir::Expression::FormatString(ir::FormatString::lower(s, context)?)
            }
            ast::Expression::Tuple(t) => {
                ir::Expression::TupleExpression(ir::TupleExpression::lower(t, context)?)
            }
            ast::Expression::ArrayRange(a) => {
                ir::Expression::ArrayExpression(ir::ArrayExpression {
                    inner: ir::ArrayExpressionInner::Range(ir::RangeExpression::lower(a, context)?),
                    unit: a
                        .unit
                        .as_ref()
                        .map(|unit| ir::Unit::lower(unit, context))
                        .transpose()?
                        .unwrap_or_default(),
                    src_ref: context.src_ref(&a.span),
                })
            }
            ast::Expression::ArrayList(a) => ir::Expression::ArrayExpression(ir::ArrayExpression {
                inner: ir::ArrayExpressionInner::List(ir::ListExpression::lower(a, context)?),
                unit: a
                    .unit
                    .as_ref()
                    .map(|ty| ir::Unit::lower(ty, context))
                    .transpose()?
                    .unwrap_or_default(),
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::QualifiedName(n) => {
                ir::Expression::QualifiedName(ir::QualifiedName::lower(n, context)?)
            }
            ast::Expression::Marker(m) => ir::Expression::Marker(ir::Marker::lower(m, context)?),
            ast::Expression::BinaryOperation(binop) => {
                ir::Expression::BinaryOp(ir::BinaryOp::lower(binop, context)?)
            }
            ast::Expression::UnaryOperation(unop) => {
                ir::Expression::UnaryOp(ir::UnaryOp::lower(unop, context)?)
            }
            ast::Expression::Body(b) => ir::Expression::Body(ir::Body::lower(b, context)?),
            ast::Expression::ElementAccess(access) => access.element_chain.iter().try_fold(
                ir::Expression::lower(&access.value, context)?,
                |acc, element| -> LowerResult<ir::Expression> {
                    use ast::ElementInner::*;

                    Ok(match &element.inner {
                        Attribute(a) => ir::Expression::AttributeAccess(
                            Box::new(acc),
                            Identifier::lower(a, context)?,
                            context.src_ref(&access.span),
                        ),
                        Tuple(t) => ir::Expression::PropertyAccess(
                            Box::new(acc),
                            Identifier::lower(t, context)?,
                            context.src_ref(&access.span),
                        ),
                        Method(m) => ir::Expression::MethodCall(
                            Box::new(acc),
                            ir::MethodCall::lower(m, context)?,
                            context.src_ref(&access.span),
                        ),
                        ArrayElement(e) => ir::Expression::ArrayElementAccess(
                            Box::new(acc),
                            Box::new(ir::Expression::lower(e, context)?),
                            context.src_ref(&access.span),
                        ),
                    })
                },
            )?,
            ast::Expression::If(i) => ir::Expression::If(Box::new(ir::If::lower(i, context)?)),
            ast::Expression::Error(span) => {
                return Err(LowerError::InvalidExpression {
                    src_ref: context.src_ref(span),
                });
            }
        })
    }
}

    */

impl Lower<ast::QualifiedName> for ir::QualifiedName {
    fn lower(node: &ast::QualifiedName, context: &mut LowerContext) -> LowerResult<Self> {
        let parts = node
            .parts
            .iter()
            .map(|ident| ir::Identifier::lower(ident, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(parts, context.src_ref(&node.span)))
    }
}

impl<EXPR> Lower<ast::TupleExpression> for ir::TupleExpression<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::TupleExpression, context: &mut LowerContext) -> LowerResult<Self> {
        let mut args = ir::ArgumentList::new();

        node.values.iter().try_for_each(|value| {
            args.try_push(
                ir::Argument {
                    id: value
                        .name
                        .as_ref()
                        .map(|name| Identifier::lower(name, context))
                        .transpose()?,
                    expression: EXPR::lower(&value.value, context)?,
                    src_ref: context.src_ref(&value.span),
                },
                context,
            )
        })?;

        Ok(Self {
            args,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower<ast::Expression> for ir::ConstantExpression {
    fn lower(node: &ast::Expression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Expression::Call(expr) => Self::Call(ir::Call::lower(expr, context)?),
            ast::Expression::Bracketed(expr, _) => Self::lower(expr, context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                span,
                ..
            }) => Self::FormatString(ir::FormatString(Refer::new(
                vec![ir::FormatStringInner::String(Refer::new(
                    s.content.clone(),
                    context.src_ref(&s.span),
                ))],
                context.src_ref(span),
            ))),
            ast::Expression::Literal(expr) => Self::Literal(ir::Literal::lower(expr, context)?),
            ast::Expression::String(s) => Self::FormatString(ir::FormatString::lower(s, context)?),
            ast::Expression::Tuple(t) => {
                Self::TupleExpression(ir::TupleExpression::lower(t, context)?)
            }
            ast::Expression::ArrayRange(a) => Self::ArrayExpression(ir::ArrayExpression {
                inner: ir::ArrayExpressionInner::Range(ir::RangeExpression::lower(a, context)?),
                unit: a
                    .unit
                    .as_ref()
                    .map(|unit| ir::Unit::lower(unit, context))
                    .transpose()?
                    .unwrap_or_default(),
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::ArrayList(a) => Self::ArrayExpression(ir::ArrayExpression {
                inner: ir::ArrayExpressionInner::List(ir::ListExpression::lower(a, context)?),
                unit: a
                    .unit
                    .as_ref()
                    .map(|ty| ir::Unit::lower(ty, context))
                    .transpose()?
                    .unwrap_or_default(),
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::QualifiedName(n) => {
                Self::QualifiedName(ir::QualifiedName::lower(n, context)?)
            }
            ast::Expression::BinaryOperation(binop) => {
                Self::BinaryOp(ir::BinaryOp::lower(binop, context)?)
            }
            ast::Expression::UnaryOperation(unop) => {
                Self::UnaryOp(ir::UnaryOp::lower(unop, context)?)
            }
            expr => {
                context.diagnostics.error(
                    &context.src_ref(&expr.span()),
                    miette::miette!("This is not a constant expression"),
                );
                Self::Invalid
            }
        })
    }
}
