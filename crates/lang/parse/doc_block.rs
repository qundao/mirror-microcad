// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
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
        assert_eq!(node.lines.len(), 1, "There should not more than 1 line");
        let line = node
            .lines
            .first()
            .cloned()
            .unwrap_or_default()
            .trim()
            .to_string();

        Ok(Self(Refer::new(line, context.src_ref(&node.span))))
    }
}
