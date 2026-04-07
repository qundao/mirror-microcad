// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::DocBlock};
use microcad_lang_base::Refer;
use microcad_syntax::ast;

impl FromAst for DocBlock {
    type AstNode = ast::Comment;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(DocBlock(Refer::new(
            match &node.inner {
                ast::CommentInner::SingleLine(items) => items.clone(),
                ast::CommentInner::MultiLine(_) => unreachable!(),
            },
            context.src_ref(&node.span),
        )))
    }
}

impl FromAst for InnerDocComment {
    type AstNode = ast::Comment;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        match &node.inner {
            ast::CommentInner::SingleLine(items) => {
                assert_eq!(items.len(), 1, "There should no more than 1 line");
                let line = items
                    .first()
                    .cloned()
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                Ok(Self(Refer::new(line, context.src_ref(&node.span))))
            }
            ast::CommentInner::MultiLine(_) => unreachable!(),
        }
    }
}
