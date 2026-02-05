// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::DocBlock};
use microcad_syntax::ast;

impl FromAst for DocBlock {
    type AstNode = ast::Comment;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let (summary, details) =
            if let Some(line_num) = node.lines.iter().position(|line| line.trim().is_empty()) {
                (
                    join_parts(&node.lines[0..line_num]),
                    Some(join_parts(&node.lines[line_num..])),
                )
            } else {
                (node.lines.clone().join("\n"), None)
            };

        Ok(DocBlock {
            summary,
            details,
            src_ref: context.src_ref(&node.span),
        })
    }
}

fn join_parts(parts: &[String]) -> String {
    let mut result = String::with_capacity(64);
    for part in parts {
        result.push_str(part);
    }
    result
}