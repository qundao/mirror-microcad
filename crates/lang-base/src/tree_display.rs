// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display trait for tree like output

/// Trait for displaying a tree
pub trait TreeDisplay {
    /// Write item into `f` and use `{:depth$}` syntax in front of your single line
    /// output to get proper indention.
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result;
}

/// Indention size
const INDENT: usize = 2;

/// Indention depth counter
#[derive(derive_more::Deref, Clone, Copy)]
pub struct TreeState {
    /// Current depth.
    #[deref]
    pub depth: usize,
    /// Print in debug mode
    pub debug: bool,
}

impl TreeState {
    /// Create new tree state for std::fmt::Display
    pub fn new_display() -> Self {
        Self {
            depth: 0,
            debug: false,
        }
    }

    /// Create new tree state for std::fmt::Debug
    pub fn new_debug(depth: usize) -> Self {
        Self { depth, debug: true }
    }
    /// Change indention one step deeper
    pub fn indent(&mut self) {
        self.depth += INDENT
    }

    /// Return a indention which is one step deeper
    pub fn indented(&self) -> Self {
        Self {
            depth: self.depth + INDENT,
            debug: self.debug,
        }
    }
}

/// print syntax via std::fmt::Display
pub struct FormatTree<'a, T: TreeDisplay>(pub &'a T);

impl<T: TreeDisplay> std::fmt::Display for FormatTree<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tree_print(
            f,
            TreeState {
                depth: 2,
                debug: false,
            },
        )
    }
}

impl<T: TreeDisplay> std::fmt::Debug for FormatTree<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tree_print(
            f,
            TreeState {
                depth: 2,
                debug: true,
            },
        )
    }
}
