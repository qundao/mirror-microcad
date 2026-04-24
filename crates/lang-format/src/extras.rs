// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_syntax::ast;

use crate::{Format, FormatConfig, Node, node};

impl Format for ast::LeadingExtras {
    fn format(&self, f: &FormatConfig) -> Node {
        let mut prev_newline = false;

        let node: Node = self
            .0
            .iter()
            .enumerate()
            .map(|(i, extra)| match &extra {
                ast::ItemExtra::Comment(comment) => {
                    let node = comment.format(f);
                    prev_newline = node.ends_with_hardline();
                    node
                }
                ast::ItemExtra::Whitespace(ws) => ws
                    .chars()
                    .filter(|&c| c == '\n')
                    .map(|_| if i > 0 { Node::Hardline } else { Node::Nil })
                    .skip(if prev_newline { 1 } else { 0 })
                    .take(2)
                    .collect::<Vec<Node>>()
                    .into(),
                _ => todo!(),
            })
            .collect::<Vec<_>>()
            .into();

        node!(f => node)
    }
}

impl Format for ast::TrailingExtras {
    fn format(&self, f: &FormatConfig) -> Node {
        let mut prev_newline = false;
        node!(
            self.0
                .iter()
                .map(|extra| match &extra {
                    ast::ItemExtra::Comment(comment) => {
                        let node = comment.format(f);
                        prev_newline = node.ends_with_hardline();
                        node
                    }
                    ast::ItemExtra::Whitespace(ws) => ws
                        .chars()
                        .filter_map(|c| {
                            if c == '\n' {
                                Some(Node::Hardline)
                            } else {
                                None
                            }
                        })
                        .skip(if prev_newline { 1 } else { 0 })
                        .take(2) // Two white spaces maximum
                        .collect::<Vec<_>>()
                        .into(),
                    _ => todo!(),
                })
                .collect::<Vec<_>>()
        )
    }
}

pub(crate) fn with_extras(
    extras: &ast::ItemExtras,
    f: &FormatConfig,
    node: impl Into<Node>,
) -> Node {
    node!(f =>
        extras.leading
        node.into()
        extras.trailing
    )
}

#[cfg(test)]
mod tests {
    use crate::{Format, FormatConfig};
    use microcad_syntax::ast;

    #[test]
    fn leading_extras() {
        //
        // // A
        //
        let extras = ast::LeadingExtras(vec![
            ast::ItemExtra::Whitespace("\n".into()),
            ast::ItemExtra::Comment(ast::Comment {
                span: 0..0,
                inner: ast::CommentInner::SingleLine("// A".into()),
            }),
            ast::ItemExtra::Whitespace("\n".into()),
        ]);

        let node = extras.format(&FormatConfig::default());
        println!("{:#?}", node)
    }
}
