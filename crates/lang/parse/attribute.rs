// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*};
use microcad_syntax::ast;

impl FromAst for AttributeCommand {
    type AstNode = ast::AttributeCommand;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(match node {
            ast::AttributeCommand::Ident(i) => {
                AttributeCommand::Ident(Identifier::from_ast(i, context)?)
            }
            ast::AttributeCommand::Assignment(a) => AttributeCommand::Assigment {
                name: Identifier::from_ast(&a.name, context)?,
                value: Expression::from_ast(&a.value, context)?,
                src_ref: context.src_ref(&a.span),
            },
            ast::AttributeCommand::Call(c) => AttributeCommand::Call(Call::from_ast(c, context)?),
        })
    }
}

impl FromAst for Attribute {
    type AstNode = ast::Attribute;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Attribute {
            src_ref: context.src_ref(&node.span),
            is_inner: node.is_inner,
            commands: node
                .commands
                .iter()
                .map(|c| AttributeCommand::from_ast(c, context))
                .collect::<Result<Vec<_>, ParseError>>()?,
        })
    }
}

impl FromAst for AttributeList {
    type AstNode = Vec<ast::Attribute>;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        node.iter()
            .map(|a| Attribute::from_ast(a, context))
            .collect::<Result<Vec<_>, _>>()
            .map(AttributeList::from)
    }
}
