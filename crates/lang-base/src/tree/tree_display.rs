// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display and Debug traits for tree-like output.

use crate::tree::NodeRef;

/// Formatting state passed down through tree nodes.
#[derive(Clone, Debug, Default)]
pub struct TreeState {
    /// String prefix applied to the current node's line.
    pub prefix: String,
}

impl TreeState {
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
        }
    }

    /// Helper to format a node and recurse through its children.
    pub fn write_node<'a, T, F, R>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        node: &NodeRef<'a, T>,
        format_content: F,
        recurse_child: R,
    ) -> std::fmt::Result
    where
        F: FnOnce(&NodeRef<'a, T>) -> String,
        R: Fn(&NodeRef<'a, T>, &mut std::fmt::Formatter<'_>, TreeState) -> std::fmt::Result,
    {
        let content = format_content(node);
        let mut lines = content.lines();

        // 1. Output first line using the current prefix (may end with ├── or └── or be empty at root)
        if let Some(first_line) = lines.next() {
            writeln!(f, "{}{}", self.prefix, first_line)?;
        } else {
            writeln!(f, "{}", self.prefix)?;
        }

        // 2. Compute continuation prefix for multiline content AND children.
        // Convert any trailing branch symbol into its continuation equivalent:
        // "├── " -> "│   "
        // "└── " -> "    "
        let continuation_prefix = if let Some(base) = self.prefix.strip_suffix("├── ") {
            format!("{base}│   ")
        } else if let Some(base) = self.prefix.strip_suffix("└── ") {
            format!("{base}    ")
        } else {
            self.prefix.clone()
        };

        // 3. Multiline lines use the continuation prefix
        for line in lines {
            writeln!(f, "{}{}", continuation_prefix, line)?;
        }

        // 4. Recurse children using continuation_prefix + branch marker
        let children: Vec<_> = node.children().collect();
        let count = children.len();

        for (idx, child) in children.into_iter().enumerate() {
            let is_last = idx == count - 1;
            let branch = if is_last { "└── " } else { "├── " };

            let child_state = TreeState {
                prefix: format!("{continuation_prefix}{branch}"),
            };

            recurse_child(&child, f, child_state)?;
        }

        Ok(())
    }
}

/// Trait for displaying a user-facing tree hierarchy.
pub trait TreeDisplay {
    fn tree_fmt(&self, f: &mut std::fmt::Formatter<'_>, state: TreeState) -> std::fmt::Result;
}

/// Trait for displaying a developer-facing diagnostic tree hierarchy.
pub trait TreeDebug {
    fn tree_debug_fmt(&self, f: &mut std::fmt::Formatter<'_>, state: TreeState)
    -> std::fmt::Result;
}
// =========================================================================
// TreeDisplay Implementation
// =========================================================================

impl<'a, T> TreeDisplay for NodeRef<'a, T>
where
    T: std::fmt::Display,
{
    fn tree_fmt(&self, f: &mut std::fmt::Formatter<'_>, state: TreeState) -> std::fmt::Result {
        state.write_node(
            f,
            self,
            |node| node.get().to_string(),
            |child, f, s| child.tree_fmt(f, s),
        )
    }
}

// =========================================================================
// TreeDebug Implementation
// =========================================================================

impl<'a, T> TreeDebug for NodeRef<'a, T>
where
    T: std::fmt::Debug,
{
    fn tree_debug_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        state: TreeState,
    ) -> std::fmt::Result {
        state.write_node(
            f,
            self,
            |node| format!("[id: {:?}] {:?}", node.id, node.get()),
            |child, f, s| child.tree_debug_fmt(f, s),
        )
    }
}

impl<'a, T> std::fmt::Display for NodeRef<'a, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_fmt(f, TreeState::new())
    }
}

impl<'a, T> std::fmt::Debug for NodeRef<'a, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_debug_fmt(f, TreeState::new())
    }
}

// =========================================================================
// Format Adapter Wrapper
// =========================================================================

/// Helper wrapper for formatting a `NodeRef` or tree item via `Display` or `Debug`.
pub struct FormatTree<'a, T>(pub &'a T);

impl<T: TreeDisplay> std::fmt::Display for FormatTree<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tree_fmt(f, TreeState::new())
    }
}

impl<T: TreeDebug> std::fmt::Debug for FormatTree<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tree_debug_fmt(f, TreeState::new())
    }
}
