// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir};

use microcad_lang_base::{Identifier, OrdMap, Refer};
use microcad_lang_parse::ast;

impl<EXPR> Lower<ast::Call> for ir::Call<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::Call, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(ir::Call {
            src_ref: context.src_ref(&node.span),
            name: ir::QualifiedName::lower(&node.name, context)?,
            argument_list: ir::ArgumentList::lower(&node.arguments, context)?,
        })
    }
}

impl<EXPR> Lower<ast::ArgumentList> for ir::ArgumentList<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::ArgumentList, context: &mut LowerContext) -> LowerResult<Self> {
        let mut argument_list =
            ir::ArgumentList(Refer::new(OrdMap::default(), context.src_ref(&node.span)));

        node.arguments.iter().try_for_each(|arg| {
            argument_list.try_push(ir::Argument::lower(arg, context)?, context)
        })?;

        Ok(argument_list)
    }
}

impl<EXPR> Lower<ast::Argument> for ir::Argument<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::Argument, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(ir::Argument {
            id: node
                .name()
                .map(|name| Identifier::lower(name, context))
                .transpose()?,
            src_ref: context.src_ref(node.span()),
            expression: EXPR::lower(node.value(), context)?,
        })
    }
}
