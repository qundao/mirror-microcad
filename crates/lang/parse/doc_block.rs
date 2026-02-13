// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::DocBlock};
use microcad_syntax::ast;

impl FromAst for DocBlock {
    type AstNode = ast::Comment;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(DocBlock(Refer::new(
            node.lines.clone(),
            context.src_ref(&node.span),
        )))
    }
}

impl FromAst for InnerDocComment {
    type AstNode = ast::Comment;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let line = if let Some(line_num) = node.lines.iter().position(|line| line.trim().is_empty())
        {
            node.lines[0..line_num].join("")
        } else {
            node.lines.clone().join("\n")
        };

        Ok(InnerDocComment(Refer::new(
            line,
            context.src_ref(&node.span),
        )))
    }
}
