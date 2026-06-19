// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};

use microcad_lang_base::{Identifier, Refer};
use microcad_lang_parse::ast;

impl Lower for ir::RangeFirst {
    type AstNode = ast::ArrayItem;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        if matches!(
            node.expr,
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
                src_ref: context.src_ref(&node.expr.span()),
            });
        }
        Ok(ir::RangeFirst(Box::new(ir::Expression::lower(
            &node.expr, context,
        )?)))
    }
}

impl Lower for ir::RangeLast {
    type AstNode = ast::ArrayItem;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        if matches!(
            node.expr,
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
                src_ref: context.src_ref(&node.expr.span()),
            });
        }
        Ok(ir::RangeLast(Box::new(ir::Expression::lower(
            &node.expr, context,
        )?)))
    }
}

impl Lower for ir::RangeExpression {
    type AstNode = ast::ArrayRangeExpression;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::RangeExpression {
            first: ir::RangeFirst::lower(&node.start, context)?,
            last: ir::RangeLast::lower(&node.end, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower for ir::ListExpression {
    type AstNode = ast::ArrayListExpression;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        node.items
            .iter()
            .map(|item| ir::Expression::lower(&item.expr, context))
            .collect::<Result<ir::ListExpression, _>>()
    }
}

impl Lower for ir::Marker {
    type AstNode = ast::Identifier;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Marker {
            id: Identifier::lower(node, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower for ir::Expression {
    type AstNode = ast::Expression;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
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
            ast::Expression::BinaryOperation(binop) => ir::Expression::BinaryOp(ir::BinaryOp {
                lhs: Box::new(ir::Expression::lower(&binop.lhs, context)?),
                rhs: Box::new(ir::Expression::lower(&binop.rhs, context)?),
                op: Refer::new(binop.op.as_str().into(), context.src_ref(&binop.op.span)),
                src_ref: context.src_ref(&binop.span),
            }),
            ast::Expression::UnaryOperation(unop) => ir::Expression::UnaryOp(ir::UnaryOp {
                rhs: Box::new(ir::Expression::lower(&unop.rhs, context)?),
                op: Refer::new(unop.op.as_str().into(), context.src_ref(&unop.op.span)),
                src_ref: context.src_ref(&unop.span),
            }),
            ast::Expression::Body(b) => ir::Expression::Body(ir::Body::lower(b, context)?),
            ast::Expression::ElementAccess(access) => access.element_chain.iter().try_fold(
                ir::Expression::lower(&access.expr, context)?,
                |acc, element| {
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

impl Lower for ir::TupleExpression {
    type AstNode = ast::TupleExpression;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        let mut args = ir::ArgumentList::default();
        for value in &node.values {
            args.value
                .try_push(ir::Argument {
                    id: value
                        .id
                        .as_ref()
                        .map(|name| Identifier::lower(name, context))
                        .transpose()?,
                    expression: ir::Expression::lower(&value.expr, context)?,
                    src_ref: context.src_ref(&value.span),
                })
                .map_err(|(previous, id)| LowerError::DuplicateArgument { previous, id })?;
        }

        Ok(ir::TupleExpression {
            args,
            src_ref: context.src_ref(&node.span),
        })
    }
}
