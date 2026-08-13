// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Index-based tree, based in crate `indextree`.

mod node_ref;
mod tree_display;

pub use indextree::*;

pub use node_ref::{NodeMut, NodeRef};

pub use tree_display::{FormatTree, TreeDisplay, TreeState};

pub struct Tree<T> {
    root: NodeId,
    arena: Arena<T>,
}

impl<T> Tree<T> {
    pub fn root<'a>(&'a self) -> NodeRef<'a, T> {
        NodeRef::new(self.root, &self.arena)
    }
}
