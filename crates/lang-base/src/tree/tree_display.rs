// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display and Debug traits for tree-like output.

use crate::{DisplayWithCtx, tree::NodeRef};

/// Formatting state passed down through tree nodes.
#[derive(Clone, Debug, Default)]
pub struct TreeState {
    /// String prefix applied to the current node's line.
    pub prefix: String,
}

impl TreeState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Trait for displaying a user-facing tree hierarchy.
pub trait TreeDisplay {
    fn tree_fmt(&self, f: &mut std::fmt::Formatter<'_>, state: TreeState) -> std::fmt::Result;
}

/// Display a tree with a context to look up symbol names.
pub trait TreeDisplayWithCtx<Ctx> {
    fn tree_fmt_with_ctx(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        ctx: &Ctx,
        state: TreeState,
    ) -> std::fmt::Result;
}

/// Trait for displaying a developer-facing diagnostic tree hierarchy.
pub trait TreeDebug {
    fn tree_debug_fmt(&self, f: &mut std::fmt::Formatter<'_>, state: TreeState)
    -> std::fmt::Result;
}
// =========================================================================
// TreeDisplay Implementation
// =========================================================================

impl<'a, Ctx, T> DisplayWithCtx<Ctx> for NodeRef<'a, T>
where
    T: DisplayWithCtx<Ctx>,
{
    fn fmt_with_ctx(&self, f: &mut std::fmt::Formatter<'_>, ctx: &Ctx) -> std::fmt::Result {
        self.write_node_with_ctx(f, self, ctx, TreeState::default())
    }
}

impl<'a, T> TreeDisplay for NodeRef<'a, T>
where
    T: std::fmt::Display,
{
    fn tree_fmt(&self, f: &mut std::fmt::Formatter<'_>, state: TreeState) -> std::fmt::Result {
        self.write_node(f, state)
    }
}

impl<'a, T, Ctx> TreeDisplayWithCtx<Ctx> for NodeRef<'a, T>
where
    T: DisplayWithCtx<Ctx>,
{
    fn tree_fmt_with_ctx(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        ctx: &Ctx,
        state: TreeState,
    ) -> std::fmt::Result {
        self.write_node_with_ctx(f, self, ctx, state)
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
        todo!()
    }
}

impl<'a, T> std::fmt::Display for NodeRef<'a, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_fmt(f, TreeState::default())
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
