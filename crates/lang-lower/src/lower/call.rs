// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, ir};

use microcad_lang_base::{Identifier, OrdMap, Refer};
use microcad_lang_parse::ast;

impl<EXPR> Lower for ir::Call<EXPR>
where
    EXPR: Lower<AstNode = ast::Expression>,
{
    type AstNode = ast::Call;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Call {
            src_ref: context.src_ref(&node.span),
            name: ir::QualifiedName::lower(&node.name, context)?,
            argument_list: ir::ArgumentList::lower(&node.arguments, context)?,
        })
    }
}

impl<EXPR> Lower for ir::ArgumentList<EXPR>
where
    EXPR: Lower<AstNode = ast::Expression>,
{
    type AstNode = ast::ArgumentList;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        let mut argument_list =
            ir::ArgumentList(Refer::new(OrdMap::default(), context.src_ref(&node.span)));
        for arg in &node.arguments {
            argument_list
                .try_push(ir::Argument::lower(arg, context)?)
                .map_err(|(previous, id)| LowerError::DuplicateArgument { previous, id })?;
        }
        Ok(argument_list)
    }
}

impl<EXPR> Lower for ir::Argument<EXPR>
where
    EXPR: Lower<AstNode = ast::Expression>,
{
    type AstNode = ast::Argument;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
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

impl<EXPR> Lower for ir::MethodCall<EXPR>
where
    EXPR: Lower<AstNode = ast::Expression>,
{
    type AstNode = ast::Call;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::MethodCall {
            name: ir::QualifiedName::lower(&node.name, context)?,
            argument_list: ir::ArgumentList::lower(&node.arguments, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
