// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::DocBlock};
use microcad_syntax::ast;

impl Parse for DocBlock {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::doc_block);

        let lines: Vec<_> = pair
            .inner()
            .filter_map(|pair| {
                if pair.as_rule() == Rule::doc_comment {
                    Some(String::from(
                        pair.as_str()
                            .trim()
                            .strip_prefix("/// ")
                            .unwrap_or_default(),
                    ))
                } else {
                    None
                }
            })
            .collect();

        let (summary, details) =
            if let Some(pos) = lines.iter().position(|line| line.trim().is_empty()) {
                (lines[0..pos].join("\n"), Some(lines[pos..].join("\n")))
            } else {
                (lines.join("\n"), None)
            };

        Ok(Self {
            summary,
            details,
            src_ref: pair.into(),
        })
    }
}

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