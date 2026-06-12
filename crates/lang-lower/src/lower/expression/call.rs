// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir, lower::sort_and_check};

use microcad_lang_base::SrcRef;
use microcad_lang_parse::ast;

impl<EXPR> Lower<ast::Call> for ir::Call<EXPR>
where
    EXPR: ir::ExpressionKind + Lower<ast::Expression>,
    EXPR::Name: Lower<ast::QualifiedName>,
{
    fn lower(node: &ast::Call, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(ir::Call {
            src_ref: context.src_ref(&node.span),
            name: EXPR::Name::lower(&node.name, context)?,
            argument_list: ir::ArgumentList::lower(&node.arguments, context)?,
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
            let expression = EXPR::lower(&arg.value, context)?;
            let src_ref = context.src_ref(&arg.span);

            match &arg.name {
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
                        src_ref: context.src_ref(&arg.span()),
                    }),
                    None => unnamed.push(ir::UnnamedArgument {
                        expression: EXPR::lower(arg.value(), context)?,
                        src_ref: context.src_ref(&arg.span()),
                    }),
                }
                Ok(())
            })?;

        Ok(Self {
            src_ref: context.src_ref(&node.span),
            unnamed_args: unnamed.into_boxed_slice(),
            named_args: sort_and_check(named, context)?,
        })
    }
}
