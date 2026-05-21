// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};

use microcad_lang_base::{Identifier, OrdMap, Refer};
use microcad_lang_parse::ast;

impl Lower for ir::Call {
    type AstNode = ast::Call;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Call {
            src_ref: context.src_ref(&node.span),
            name: ir::QualifiedName::lower(&node.name, context)?,
            argument_list: ir::ArgumentList::lower(&node.arguments, context)?,
        })
    }
}

impl Lower for ir::ArgumentList {
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

impl Lower for ir::Argument {
    type AstNode = ast::Argument;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Argument {
            id: node
                .name()
                .map(|name| Identifier::lower(name, context))
                .transpose()?,
            src_ref: context.src_ref(node.span()),
            expression: ir::Expression::lower(node.value(), context)?,
        })
    }
}

impl Lower for ir::MethodCall {
    type AstNode = ast::Call;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::MethodCall {
            name: ir::QualifiedName::lower(&node.name, context)?,
            argument_list: ir::ArgumentList::lower(&node.arguments, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
