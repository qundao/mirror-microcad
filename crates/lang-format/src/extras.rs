// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_syntax::ast;

use crate::{Format, FormatConfig, Node, node};

pub(crate) fn leading_extras_without_newline(extras: &ast::ItemExtras) -> ast::ItemExtras {
    let mut leading = extras.leading.clone();

    if let Some(first_item) = leading.0.first_mut() {
        if let ast::ItemExtra::Whitespace(ws) = first_item {
            // Find the index of the first newline
            if let Some(pos) = ws.find('\n') {
                // Remove only that specific character
                ws.remove(pos);
            }
        }
    }

    ast::ItemExtras {
        leading,
        trailing: extras.trailing.clone(),
    }
}

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
                    .map(|_| Node::Hardline)
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
