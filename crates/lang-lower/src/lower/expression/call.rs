// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Lower, LowerContext, LowerResult, ir,
    lower::{LowerExpr, LowerName, function::builtin_fn, sort_and_check},
};

use microcad_lang_base::{SpanToSrcRef, SrcRef};
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
                None => unnamed.push(ir::UnnamedArgument {
                    expression,
                    src_ref,
                }),
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
                    None => unnamed.push(ir::UnnamedArgument {
                        expression: EXPR::lower(arg.value(), context)?,
                        src_ref: context.span_to_src_ref(arg.span()),
                    }),
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
            name: builtin_fn(node.op.to_fn_name()),
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
            name: builtin_fn(node.op.to_fn_name()),
            args: ir::ArgumentList::from_iter([
                Expr::lower(&node.lhs, context)?,
                Expr::lower(&node.rhs, context)?,
            ]),
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}
