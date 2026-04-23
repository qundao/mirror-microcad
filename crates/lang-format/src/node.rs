// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use compact_str::{CompactString, ToCompactString};

use crate::{FormatConfig, node};

/// How to perform a line break
pub enum BreakMode {
    NoBreak,
    WithIndent(usize),
}

impl BreakMode {
    /// Determines the break strategy based on the format config
    pub fn from_layout(nodes: &[Node], max_items: usize, f: &FormatConfig) -> Self {
        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        let too_many_items = max_items > 0 && nodes.len() > max_items;
        let too_wide = width > f.max_width;
        let forced_break = nodes.iter().any(|node| node.contains_hardline());

        if too_many_items || too_wide || forced_break {
            Self::WithIndent(f.indent_width)
        } else {
            Self::NoBreak
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Node {
    #[default]
    Nil,
    Text(CompactString),
    // A comment starting with `//`
    SingleLineComment(CompactString),
    Hardline,
    Softline,
    Indent {
        width: usize,
        node: Box<Node>,
    },
    Group(Vec<Node>),
}

impl Node {
    /// Intersperses a separator between nodes from any iterable source.
    pub fn hlist<I>(nodes: I, separator: impl Into<Node>) -> Node
    where
        I: IntoIterator<Item = Node>,
    {
        let iter = nodes.into_iter();
        let separator: Node = separator.into();
        // Provide a hint to the allocator if possible
        let (lower, _) = iter.size_hint();
        let mut result = Vec::with_capacity(lower.saturating_mul(2));

        let mut first = true;
        for node in iter {
            if !first {
                result.push(separator.clone());
            }
            result.push(node);
            first = false;
        }

        result.into()
    }

    pub fn vlist<I>(nodes: I, separator: impl Into<Node>, indent_width: usize) -> Node
    where
        I: IntoIterator<Item = Node>,
    {
        let sep = separator.into();
        let nodes: Node = nodes
            .into_iter()
            .flat_map(|node| vec![node, sep.clone(), Node::Hardline])
            .collect::<Vec<_>>()
            .into();
        match indent_width {
            0 => nodes,
            width => node!(
                Node::Hardline
                Node::Indent { width, node: Box::new(nodes) }
            ),
        }
    }

    /// A list of items with an separator
    pub fn list<I>(nodes: I, separator: impl Into<Node>, break_mode: BreakMode) -> Node
    where
        I: IntoIterator<Item = Node>,
    {
        let sep = separator.into();
        match break_mode {
            BreakMode::NoBreak => Self::hlist(nodes, node!(sep ' ')),
            BreakMode::WithIndent(indent_width) => Self::vlist(nodes, sep, indent_width),
        }
    }

    pub fn estimate_width(&self) -> usize {
        match &self {
            Node::Nil => 0,
            Node::Text(compact_string) => compact_string.len(),
            Node::Hardline | Node::SingleLineComment(_) => 0,
            Node::Softline => 1,
            Node::Indent { width, node } => width + node.estimate_width(),
            Node::Group(group) => group
                .iter()
                .map(|node| node.estimate_width())
                .max()
                .unwrap_or_default(),
        }
    }

    pub fn indent(width: usize, node: impl Into<Node>) -> Self {
        Node::Indent {
            width,
            node: Box::new(node.into()),
        }
    }

    pub fn contains_hardline(&self) -> bool {
        match &self {
            Node::Nil | Node::Softline => false,
            Node::Text(compact_string) => compact_string.contains("\n"),
            Node::Hardline | Node::SingleLineComment(_) => true,
            Node::Indent { width: _, node } => node.contains_hardline(),
            Node::Group(group) => group.iter().any(|node| node.contains_hardline()),
        }
    }

    /// Compact nodes: Remove Nil nodes and concatenate adjacent Text ndoes
    pub fn compact(nodes: Vec<Node>) -> Self {
        // 1. Filter out Nil and
        // 2. flatten nested Groups
        // 3. Merge adjacent Text nodes
        let mut compacted = Vec::with_capacity(nodes.len());

        for node in nodes {
            match (compacted.last_mut(), node) {
                (_, Node::Nil) => continue,
                // Merge adjacent text nodes into one
                (Some(Node::Text(last)), Node::Text(next)) => {
                    last.push_str(&next);
                }
                (_, Node::Group(group)) => {
                    let flattened = Self::compact(group);
                    match flattened {
                        Node::Group(mut sub_vec) => compacted.append(&mut sub_vec),
                        Node::Nil => continue,
                        node => {
                            if let (Some(Node::Text(last)), Node::Text(next)) =
                                (compacted.last_mut(), &node)
                            {
                                last.push_str(next);
                            } else {
                                compacted.push(node);
                            }
                        }
                    }
                }
                // Otherwise, add the node
                (_, other) => compacted.push(other),
            }
        }

        // 4. Handle the resulting length
        match compacted.len() {
            0 => Node::Nil,
            1 => compacted.remove(0),
            _ => Node::Group(compacted),
        }
    }
}

impl From<Vec<Node>> for Node {
    fn from(nodes: Vec<Node>) -> Self {
        Node::compact(nodes)
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Text(value.to_compact_string())
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Text(value.to_compact_string())
    }
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        Node::Text(value.to_compact_string())
    }
}

impl From<CompactString> for Node {
    fn from(value: CompactString) -> Self {
        Node::Text(value)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We initialize a tiny state tracker for the recursive process
        let mut state = RenderState {
            indent_level: 0,
            column: 0,
            indent_pending: false,
            extra_pending: false,
        };
        self.render_recursive(f, &mut state)
    }
}

struct RenderState {
    indent_level: usize,
    column: usize,
    indent_pending: bool,
    extra_pending: bool,
}

impl Node {
    fn render_recursive(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        state: &mut RenderState,
    ) -> std::fmt::Result {
        fn write_pending_indent(
            f: &mut std::fmt::Formatter<'_>,
            state: &mut RenderState,
        ) -> std::fmt::Result {
            if state.indent_pending {
                let spaces = " ".repeat(state.indent_level);
                state.indent_pending = false;
                write!(f, "{spaces}")
            } else {
                Ok(())
            }
        }

        fn write_extra_pending(
            f: &mut std::fmt::Formatter<'_>,
            state: &mut RenderState,
        ) -> std::fmt::Result {
            if state.extra_pending {
                state.extra_pending = false;
                state.column = state.indent_level;
                writeln!(f)
            } else {
                Ok(())
            }
        }

        match self {
            Node::Text(s) => {
                write_extra_pending(f, state)?;
                write_pending_indent(f, state)?;
                state.column += s.len();
                write!(f, "{s}")
            }
            Node::SingleLineComment(s) => {
                write_extra_pending(f, state)?;
                write_pending_indent(f, state)?;
                if state.column as i32 - state.indent_level as i32 > 0 {
                    write!(f, " ")?; // Leading ws
                }
                state.column += s.len();
                state.indent_pending = true;
                state.extra_pending = true;
                write!(f, "{s}")
            }
            Node::Hardline => {
                state.column = state.indent_level;
                state.indent_pending = true;
                writeln!(f)
            }
            Node::Softline => {
                write_pending_indent(f, state)?;
                if state.column as i32 - state.indent_level as i32 > 0 {
                    write!(f, " ") // Leading ws
                } else {
                    write!(f, "")
                }
            }
            Node::Group(group) => {
                write_extra_pending(f, state)?;
                write_pending_indent(f, state)?;
                group
                    .iter()
                    .try_for_each(|node| node.render_recursive(f, state))
            }
            Node::Indent { width, node } => {
                state.indent_level += width;
                node.render_recursive(f, state)?;
                state.indent_level -= width;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
