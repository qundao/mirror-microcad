// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display and Debug traits for tree-like output.

use crate::tree::NodeRef;

/// Indentation size (number of spaces per depth level)
const INDENT_SIZE: usize = 2;

/// Formatting state passed down through tree nodes.
#[derive(Clone, Copy, Debug, Default)]
pub struct TreeState {
    /// Current tree depth (0 = root level).
    pub level: usize,
}

impl TreeState {
    pub fn new() -> Self {
        Self { level: 0 }
    }

    /// Total spaces required for current level padding.
    pub fn indent_spaces(&self) -> usize {
        self.level * INDENT_SIZE
    }

    /// Returns a new state incremented by one depth level.
    pub fn indented(&self) -> Self {
        Self {
            level: self.level + 1,
        }
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
        writeln!(
            f,
            "{:indent$}{}",
            "",
            self.get(),
            indent = state.indent_spaces()
        )?;

        self.children()
            .try_for_each(|child| child.tree_fmt(f, state.indented()))
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
        // Includes NodeId for deep CAD debugging
        writeln!(
            f,
            "{:indent$}[id: {:?}]{:?}",
            "",
            self.id,
            self.get(),
            indent = state.indent_spaces()
        )?;

        self.children()
            .try_for_each(|child| child.tree_debug_fmt(f, state.indented()))
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
