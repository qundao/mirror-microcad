// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Index-based tree, based in crate `indextree`.

mod node_ref;
mod tree_display;

pub use indextree::*;

pub use node_ref::{NodeMut, NodeRef};

use serde::{Deserialize, Serialize};
pub use tree_display::{FormatTree, TreeDisplay, TreeState};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tree<T> {
    root: NodeId,
    arena: Arena<T>,
}

impl<T> Tree<T> {
    pub fn new(root: NodeId, arena: Arena<T>) -> Self {
        Self { root, arena }
    }

    pub fn root<'a>(&'a self) -> NodeRef<'a, T> {
        NodeRef::new(self.root, &self.arena)
    }
}

impl<T> std::hash::Hash for Tree<T>
where
    T: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.root().descendants().for_each(|node| node.hash(state));
    }
}
