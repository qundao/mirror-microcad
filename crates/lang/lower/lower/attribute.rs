// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};

use microcad_lang_base::Identifier;
use microcad_lang_parse::ast;

impl Lower for ir::AttributeCommand {
    type AstNode = ast::AttributeCommand;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(match node {
            ast::AttributeCommand::Ident(i) => {
                ir::AttributeCommand::Ident(Identifier::lower(i, context)?)
            }
            ast::AttributeCommand::Assignment(a) => ir::AttributeCommand::Assignment {
                name: Identifier::lower(&a.id, context)?,
                value: ir::Expression::lower(&a.expr, context)?,
                src_ref: context.src_ref(&a.span),
            },
            ast::AttributeCommand::Call(c) => {
                ir::AttributeCommand::Call(ir::Call::lower(c, context)?)
            }
        })
    }
}

impl Lower for ir::Attribute {
    type AstNode = ast::Attribute;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Attribute {
            src_ref: context.src_ref(&node.span),
            is_inner: node.is_inner,
            commands: node
                .commands
                .iter()
                .map(|c| ir::AttributeCommand::lower(c, context))
                .collect::<Result<Vec<_>, LowerError>>()?,
        })
    }
}

impl Lower for ir::AttributeList {
    type AstNode = Vec<ast::Attribute>;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        node.iter()
            .map(|a| ir::Attribute::lower(a, context))
            .collect::<Result<Vec<_>, _>>()
            .map(ir::AttributeList::from)
    }
}
