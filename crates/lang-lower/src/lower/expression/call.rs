// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Lower, LowerContext, LowerResult, ir,
    lower::{LowerExpr, LowerName, sort_and_check},
};

use microcad_lang_base::{__mu, SpanToSrcRef, SrcRef};
use microcad_lang_parse::ast;

impl<EXPR: LowerExpr> Lower<ast::Call> for ir::Call<EXPR>
where
    EXPR::Name: LowerName,
{
    fn lower(node: &ast::Call, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(ir::Call {
            src_ref: context.span_to_src_ref(&node.span),
            name: EXPR::Name::lower(&node.name, context)?,
            args: ir::ArgumentList::lower(&node.arguments, context)?,
        })
    }
}

impl<EXPR> Lower<Vec<ast::TupleItem>> for ir::ArgumentList<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &Vec<ast::TupleItem>, context: &mut LowerContext) -> LowerResult<Self> {
        let mut unnamed = Vec::new();
        let mut named = Vec::new();

        node.iter().try_for_each(|arg| -> LowerResult<()> {
            let expression = EXPR::lower(&arg.expr, context)?;
            let src_ref = context.span_to_src_ref(&arg.span);

            match &arg.id {
                Some(name) => named.push(ir::NamedArgument {
                    id: ir::Identifier::lower(name, context)?,
                    expression,
                    src_ref,
                }),
                None => unnamed.push(expression),
            }
            Ok(())
        })?;

        Ok(Self {
            src_ref: SrcRef::none(),
            unnamed_args: unnamed.into_boxed_slice(),
            named_args: sort_and_check(named, context)?,
        })
    }
}

impl<EXPR> Lower<ast::ArgumentList> for ir::ArgumentList<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::ArgumentList, context: &mut LowerContext) -> LowerResult<Self> {
        let mut unnamed = Vec::new();
        let mut named = Vec::new();

        node.arguments
            .iter()
            .try_for_each(|arg| -> LowerResult<()> {
                match arg.name() {
                    Some(name) => named.push(ir::NamedArgument {
                        id: ir::Identifier::lower(name, context)?,
                        expression: EXPR::lower(arg.value(), context)?,
                        src_ref: context.span_to_src_ref(arg.span()),
                    }),
                    None => unnamed.push(EXPR::lower(arg.value(), context)?),
                }
                Ok(())
            })?;

        Ok(Self {
            src_ref: context.span_to_src_ref(&node.span),
            unnamed_args: unnamed.into_boxed_slice(),
            named_args: sort_and_check(named, context)?,
        })
    }
}

impl<Expr: LowerExpr> Lower<ast::UnaryOperation> for ir::Call<Expr>
where
    Expr::Name: LowerName,
{
    fn lower(node: &ast::UnaryOperation, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            name: __mu(node.op.to_fn_name()).into(),
            args: ir::ArgumentList::from_iter([Expr::lower(&node.rhs, context)?]),
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl<Expr: LowerExpr> Lower<ast::BinaryOperation> for ir::Call<Expr>
where
    Expr::Name: LowerName,
{
    fn lower(node: &ast::BinaryOperation, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            name: __mu(node.op.to_fn_name()).into(),
            args: ir::ArgumentList::from_iter([
                Expr::lower(&node.lhs, context)?,
                Expr::lower(&node.rhs, context)?,
            ]),
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

pub fn lower_spec<Name: LowerName>(
    expr: ir::ConstantExpression<Name>,
    spec: &ast::StringFormatSpecification,
    context: &mut LowerContext,
) -> LowerResult<ir::Call<ir::ConstantExpression<Name>>> {
    let width = spec
        .width
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .copied()
        .map(|w| ir::ConstantExpression::Literal(ir::Literal::from_value(w as i64)))
        .unwrap_or(ir::ConstantExpression::Invalid);

    let precision = spec
        .precision
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .copied()
        .map(|p| ir::ConstantExpression::Literal(ir::Literal::from_value(p as i64)))
        .unwrap_or(ir::ConstantExpression::Invalid);

    Ok(ir::Call {
        name: __mu("core::format_spec").into(),
        args: ir::ArgumentList::from_iter([expr, width, precision]),
        src_ref: context.span_to_src_ref(&spec.span),
    })
}

impl<Name: LowerName> Lower<ast::FormatString> for ir::Call<ir::ConstantExpression<Name>> {
    fn lower(node: &ast::FormatString, context: &mut LowerContext) -> LowerResult<Self> {
        let mut args_vec = Vec::new();
        let mut pending_str = String::new();

        for part in &node.parts {
            match part {
                ast::StringPart::Char(c) => {
                    pending_str.push(c.character);
                }
                ast::StringPart::Content(lit) => {
                    pending_str.push_str(&lit.content);
                }
                ast::StringPart::Expression(expr_part) => {
                    // Flush accumulated string literal first
                    if !pending_str.is_empty() {
                        args_vec.push(ir::ConstantExpression::Literal(ir::Literal::from_value(
                            std::mem::take(&mut pending_str),
                        )));
                    }

                    let lowered_expr =
                        ir::ConstantExpression::lower(expr_part.expr.as_ref(), context)?;

                    if expr_part.specification.is_some() {
                        args_vec.push(
                            lower_spec(lowered_expr, &expr_part.specification, context)?.into(),
                        );
                    } else {
                        args_vec.push(lowered_expr);
                    }
                }
            }
        }

        // Flush remaining trailing string content
        if !pending_str.is_empty() {
            args_vec.push(ir::ConstantExpression::Literal(ir::Literal::from_value(
                pending_str,
            )));
        }

        Ok(Self {
            name: __mu("core::format").into(),
            args: ir::ArgumentList::from_iter(args_vec),
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}
