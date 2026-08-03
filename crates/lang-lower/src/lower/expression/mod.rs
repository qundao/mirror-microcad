// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Lower, LowerContext, LowerError, LowerResult, ir,
    lower::{LowerExpr, LowerName},
};

mod call;
mod literal;

use microcad_lang_base::{__mu, Identifier, SpanToSrcRef};
use microcad_lang_parse::ast;
use microcad_lang_types::{BinaryOperator, Scalar, Value};

impl Lower<ast::Identifier> for ir::Marker {
    fn lower(node: &ast::Identifier, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            id: Identifier::lower(node, context)?,
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl<EXPR: ir::ExpressionKind> Lower<ast::If> for ir::If<EXPR>
where
    EXPR: Lower<ast::Expression>,
    EXPR::Body: Lower<ast::Body>,
{
    fn lower(node: &ast::If, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(ir::If {
            if_ref: context.span_to_src_ref(&node.if_span),
            cond: Box::new(EXPR::lower(node.condition.as_ref(), context)?),
            body: EXPR::Body::lower(&node.body, context)?.into(),
            next_if_ref: node
                .next_if_span
                .as_ref()
                .map(|span| context.span_to_src_ref(span)),
            next_if: node
                .next_if
                .as_ref()
                .map(|next| ir::If::lower(next, context))
                .transpose()?
                .map(Box::new),
            else_ref: node
                .else_span
                .as_ref()
                .map(|span| context.span_to_src_ref(span)),
            body_else: node
                .else_body
                .as_ref()
                .map(|body| EXPR::Body::lower(body, context))
                .transpose()?
                .map(Box::new),
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl Lower<ast::SymbolPath> for ir::SymbolPath {
    fn lower(node: &ast::SymbolPath, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self::Path {
            is_absolute: node.prefix.is_some(),
            parts: node
                .parts
                .iter()
                .map(|ident| ir::Identifier::lower(ident, context))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl<EXPR> Lower<ast::TupleExpression> for ir::TupleExpression<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::TupleExpression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            args: ir::ArgumentList::lower(&node.values, context)?,
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl<Expr: LowerExpr> Lower<ast::ArrayRangeExpression> for Expr
where
    Expr: From<ir::Call<Expr>> + From<ir::Literal>,
    Expr::Name: LowerName,
{
    fn lower(a: &ast::ArrayRangeExpression, context: &mut LowerContext) -> LowerResult<Self> {
        let unit = ir::Unit::lower(&a.unit, context)?;
        let range = Expr::from(ir::Call {
            name: __mu("core::range").into(),
            args: ir::ArgumentList::from_iter([
                Expr::lower(&a.start.expr, context)?,
                Expr::lower(&a.end.expr, context)?,
            ]),
            src_ref: context.span_to_src_ref(&a.span),
        });

        Ok(if unit.is_none() {
            range
        } else {
            Expr::from(ir::Call {
                name: __mu(BinaryOperator::Multiply.to_fn_name()).into(),
                args: ir::ArgumentList::from_iter([
                    range,
                    Expr::from(ir::Literal::from(
                        (Value::from(Scalar::from_num(1.0)) * unit)?,
                    )),
                ]),
                src_ref: context.span_to_src_ref(&a.span),
            })
        })
    }
}

impl<Expr: LowerExpr> Lower<ast::ArrayListExpression> for Expr
where
    Expr: From<ir::Call<Expr>> + From<ir::Literal>,
    Expr::Name: LowerName,
{
    fn lower(a: &ast::ArrayListExpression, context: &mut LowerContext) -> LowerResult<Self> {
        let unit = ir::Unit::lower(&a.unit, context)?;

        let args = a
            .items
            .iter()
            .map(|item| Expr::lower(&item.expr, context))
            .collect::<Result<Vec<Expr>, _>>()?;

        let list = Expr::from(ir::Call {
            name: __mu("core::list").into(),
            args: ir::ArgumentList::from_iter(args),
            src_ref: context.span_to_src_ref(&a.span),
        });

        Ok(if unit.is_none() {
            list
        } else {
            Expr::from(ir::Call {
                name: __mu(BinaryOperator::Multiply.to_fn_name()).into(),
                args: ir::ArgumentList::from_iter([
                    list,
                    Expr::from(ir::Literal::from(
                        (Value::from(Scalar::from_num(1.0)) * unit)?,
                    )),
                ]),
                src_ref: context.span_to_src_ref(&a.span),
            })
        })
    }
}

impl<NAME: LowerName> Lower<ast::Expression> for ir::ConstantExpression<NAME> {
    fn lower(node: &ast::Expression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Expression::Bracketed(expr, _) => Self::lower(expr.as_ref(), context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                ..
            }) => Self::Literal(ir::Literal::from_value(s.content.clone())),
            ast::Expression::Literal(expr) => Self::Literal(ir::Literal::lower(expr, context)?),
            ast::Expression::String(s) => Self::Call(ir::Call::lower(s, context)?),
            ast::Expression::Tuple(t) => Self::Tuple(ir::TupleExpression::lower(t, context)?),
            ast::Expression::ArrayRange(a) => Self::lower(a, context)?,
            ast::Expression::ArrayList(a) => Self::lower(a, context)?,
            ast::Expression::SymbolPath(n) => Self::Name(NAME::lower(n, context)?),
            ast::Expression::BinaryOperation(binop) => Self::Call(ir::Call::lower(binop, context)?),
            ast::Expression::UnaryOperation(unop) => Self::Call(ir::Call::lower(unop, context)?),
            expr => {
                context.diag(LowerError::InvalidConstantExpression {
                    src_ref: context.span_to_src_ref(&expr.span()),
                });
                Self::Invalid
            }
        })
    }
}
