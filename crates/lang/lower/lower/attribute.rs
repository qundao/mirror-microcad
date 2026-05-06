// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};

use microcad_lang_base::Identifier;
use microcad_syntax::ast;

impl FromAst for ir::AttributeCommand {
    type AstNode = ast::AttributeCommand;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(match node {
            ast::AttributeCommand::Ident(i) => {
                ir::AttributeCommand::Ident(Identifier::from_ast(i, context)?)
            }
            ast::AttributeCommand::Assignment(a) => ir::AttributeCommand::Assignment {
                name: Identifier::from_ast(&a.name, context)?,
                value: ir::Expression::from_ast(&a.value, context)?,
                src_ref: context.src_ref(&a.span),
            },
            ast::AttributeCommand::Call(c) => {
                ir::AttributeCommand::Call(ir::Call::from_ast(c, context)?)
            }
        })
    }
}

impl FromAst for ir::Attribute {
    type AstNode = ast::Attribute;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Attribute {
            src_ref: context.src_ref(&node.span),
            is_inner: node.is_inner,
            commands: node
                .commands
                .iter()
                .map(|c| ir::AttributeCommand::from_ast(c, context))
                .collect::<Result<Vec<_>, LowerError>>()?,
        })
    }
}

impl FromAst for ir::AttributeList {
    type AstNode = Vec<ast::Attribute>;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        node.iter()
            .map(|a| ir::Attribute::from_ast(a, context))
            .collect::<Result<Vec<_>, _>>()
            .map(ir::AttributeList::from)
    }
}
