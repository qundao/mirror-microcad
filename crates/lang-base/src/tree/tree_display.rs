// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display trait for tree-like output

use std::fmt;

/// Trait for displaying a tree hierarchy.
pub trait TreeDisplay {
    /// Write item into `f` using the current `state` for padding and formatting.
    fn tree_print(&self, f: &mut fmt::Formatter<'_>, state: TreeState) -> fmt::Result;
}

/// Indentation size (number of spaces per depth level)
const INDENT_SIZE: usize = 2;

/// Formatting state passed down through tree nodes
#[derive(Clone, Copy, Debug)]
pub struct TreeState {
    /// Current tree depth (0 = root level).
    pub level: usize,
    /// Whether to print in debug mode.
    pub debug: bool,
}

impl TreeState {
    pub fn new_display() -> Self {
        Self {
            level: 0,
            debug: false,
        }
    }

    pub fn new_debug(level: usize) -> Self {
        Self { level, debug: true }
    }

    /// Total spaces required for current level padding.
    pub fn indent_spaces(&self) -> usize {
        self.level * INDENT_SIZE
    }

    /// Returns a new state incremented by one depth level.
    pub fn indented(&self) -> Self {
        Self {
            level: self.level + 1,
            debug: self.debug,
        }
    }
}

/// Helper wrapper for formatting a `TreeDisplay` item via `Display` or `Debug`.
pub struct FormatTree<'a, T: TreeDisplay>(pub &'a T);

impl<T: TreeDisplay> fmt::Display for FormatTree<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.tree_print(f, TreeState::new_display())
    }
}

impl<T: TreeDisplay> fmt::Debug for FormatTree<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.tree_print(f, TreeState::new_debug(0))
    }
}
