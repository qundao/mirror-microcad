// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use compact_str::{CompactString, ToCompactString};

use crate::node;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Node {
    #[default]
    Nil,
    Text(CompactString),
    Hardline,
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
    pub fn list<I>(
        nodes: I,
        separator: impl Into<Node>,
        hardline: bool,
        indent_width: usize,
    ) -> Node
    where
        I: IntoIterator<Item = Node>,
    {
        let sep = separator.into();
        if hardline {
            Self::vlist(nodes, sep, indent_width)
        } else {
            Self::hlist(nodes, node!(sep ' '))
        }
    }

    pub fn estimate_width(&self) -> usize {
        match &self {
            Node::Nil => 0,
            Node::Text(compact_string) => compact_string.len(),
            Node::Hardline => 0,
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
            Node::Nil => false,
            Node::Text(compact_string) => compact_string.contains("\n"),
            Node::Hardline => true,
            Node::Indent { width: _, node } => node.contains_hardline(),
            Node::Group(group) => group.iter().any(|node| node.contains_hardline()),
        }
    }

    pub fn ends_with_hardline(&self) -> bool {
        match &self {
            Node::Nil => false,
            Node::Text(compact_string) => compact_string.ends_with("\n"),
            Node::Hardline => true,
            Node::Indent { width: _, node } => node.ends_with_hardline(),
            Node::Group(group) => group
                .last()
                .map(|node| node.ends_with_hardline())
                .unwrap_or_default(),
        }
    }
}

impl From<Vec<Node>> for Node {
    fn from(nodes: Vec<Node>) -> Self {
        let nodes = nodes
            .into_iter()
            .filter(|node| !matches!(node, Node::Nil))
            .collect::<Vec<_>>();
        match nodes.len() {
            0 => Node::Nil,
            1 => nodes.first().expect("Some node").clone(),
            _ => Node::Group(nodes),
        }
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
        };
        self.render_recursive(f, &mut state)
    }
}

struct RenderState {
    indent_level: usize,
    column: usize,
    indent_pending: bool,
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
                write!(f, "{}", spaces)
            } else {
                Ok(())
            }
        }

        match self {
            Node::Text(s) => {
                write_pending_indent(f, state)?;
                state.column += s.len();
                write!(f, "{}", s)
            }
            Node::Hardline => {
                state.column = state.indent_level;
                state.indent_pending = true;
                writeln!(f)
            }
            Node::Group(group) => {
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
