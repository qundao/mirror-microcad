// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use compact_str::{CompactString, ToCompactString};

pub struct DocBuilder {
    nodes: Vec<Node>,
}

impl DocBuilder {
    /// Start a new builder
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Add a raw text segment
    pub fn text(mut self, text: impl Into<CompactString>) -> Self {
        self.nodes.push(Node::Text(text.into()));
        self
    }

    /// Add a hard line break (always breaks)
    pub fn hardline(mut self) -> Self {
        self.nodes.push(Node::Hardline);
        self
    }

    /// Add a soft line break (breaks only if the group doesn't fit)
    pub fn softline(mut self) -> Self {
        self.nodes.push(Node::Softline);
        self
    }

    pub fn indent(mut self, width: usize, node: impl Into<Node>) -> Self {
        self.nodes.push(Node::Indent {
            width,
            node: Box::new(node.into()),
        });
        self
    }

    /// Wrap a set of nodes into a Group
    pub fn group(mut self, inner: DocBuilder) -> Self {
        self.nodes.push(inner.build_vec().into());
        self
    }

    /// Finalize into a single Node (usually a Group or a list)
    pub fn build(self) -> Node {
        self.nodes.into()
    }

    fn build_vec(self) -> Vec<Node> {
        self.nodes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BreakMode {
    /// Forces all elements to stay on one line.
    Never,
    /// Forces each element onto its own line with indentation.
    Always,
    /// (Optional) The standard Wadler-Leijen style:
    /// Break only if it exceeds max width.
    Fit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    /// The actual AST nodes (e.g., the elements of the array).
    pub nodes: Vec<Node>,

    /// The "glue" placed between nodes (e.g., Node::Text(", ")).
    pub separator: Option<Box<Node>>,

    /// The structural strategy for this specific group.
    pub break_mode: BreakMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Nil,
    Text(CompactString),
    Hardline,
    Softline,
    Indent { width: usize, node: Box<Node> },
    Group(Group),
}

impl Node {
    /// Intersperses a separator between nodes from any iterable source.
    pub fn interspersed<I>(nodes: I, separator: impl Into<Node>) -> Node
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

    pub fn estimate_width(&self) -> usize {
        match &self {
            Node::Nil => 0,
            Node::Text(compact_string) => compact_string.len(),
            Node::Hardline => 0,
            Node::Softline => 1,
            Node::Indent { width, node } => width + node.estimate_width(),
            Node::Group(group) => group
                .nodes
                .iter()
                .map(|node| node.estimate_width())
                .max()
                .unwrap_or_default(),
        }
    }
}

impl From<Vec<Node>> for Node {
    fn from(value: Vec<Node>) -> Self {
        match value.len() {
            0 => Node::Nil,
            1 => value.first().expect("Some node").clone(),
            _ => Node::Group(Group {
                nodes: value,
                separator: None,
                break_mode: BreakMode::Fit,
            }),
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
        match self {
            Node::Text(s) => {
                if state.indent_pending {
                    let spaces = " ".repeat(state.indent_level);
                    write!(f, "{}", spaces)?;
                    state.indent_pending = false;
                }
                write!(f, "{}", s)?;
                state.column += s.len();
            }
            Node::Hardline => {
                writeln!(f)?;
                state.column = state.indent_level;
                state.indent_pending = true;
            }
            Node::Group(group) => {
                if state.indent_pending {
                    let spaces = " ".repeat(state.indent_level);
                    write!(f, "{}", spaces)?;
                    state.indent_pending = false;
                }

                let len = group.nodes.len();
                for (i, node) in group.nodes.iter().enumerate() {
                    node.render_recursive(f, state)?;

                    if i < len - 1 {
                        if let Some(sep) = &group.separator {
                            sep.render_recursive(f, state)?;
                        }
                        if group.break_mode == BreakMode::Always {
                            Node::Hardline.render_recursive(f, state)?;
                        }
                    } else if group.break_mode == BreakMode::Always {
                        // Trailing separator logic for multi-line
                        if let Some(sep) = &group.separator {
                            sep.render_recursive(f, state)?;
                        }
                        Node::Hardline.render_recursive(f, state)?;
                    }
                }
            }
            Node::Indent { width, node } => {
                state.indent_level += width;
                node.render_recursive(f, state)?;
                state.indent_level -= width;
            }
            _ => {}
        }
        Ok(())
    }
}
