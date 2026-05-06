// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};

use microcad_lang_base::{Identifier, OrdMap, Refer};
use microcad_syntax::ast;

impl FromAst for ir::Call {
    type AstNode = ast::Call;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Call {
            src_ref: context.src_ref(&node.span),
            name: ir::QualifiedName::from_ast(&node.name, context)?,
            argument_list: ir::ArgumentList::from_ast(&node.arguments, context)?,
        })
    }
}

impl FromAst for ir::ArgumentList {
    type AstNode = ast::ArgumentList;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        let mut argument_list =
            ir::ArgumentList(Refer::new(OrdMap::default(), context.src_ref(&node.span)));
        for arg in &node.arguments {
            argument_list
                .try_push(ir::Argument::from_ast(arg, context)?)
                .map_err(|(previous, id)| LowerError::DuplicateArgument { previous, id })?;
        }
        Ok(argument_list)
    }
}

impl FromAst for ir::Argument {
    type AstNode = ast::Argument;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Argument {
            id: node
                .name()
                .map(|name| Identifier::from_ast(name, context))
                .transpose()?,
            src_ref: context.src_ref(node.span()),
            expression: ir::Expression::from_ast(node.value(), context)?,
        })
    }
}

impl FromAst for ir::MethodCall {
    type AstNode = ast::Call;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::MethodCall {
            name: ir::QualifiedName::from_ast(&node.name, context)?,
            argument_list: ir::ArgumentList::from_ast(&node.arguments, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
