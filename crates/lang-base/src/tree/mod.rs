// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Index-based tree, based in crate `indextree`.

mod node_ref;
mod tree_display;

pub use indextree::*;

pub use node_ref::{NodeMut, NodeRef};

pub use tree_display::{FormatTree, TreeDisplay, TreeState};

#[macro_export]
macro_rules! impl_tree_types {
    ($vis:vis $tree:ident<$item:ty>) => {
        impl $tree {
            pub fn new(root: $item) -> Self {
                let mut arena = Arena::default();
                let root = arena.new_node(root);
                Self { root, arena }
            }

            pub fn root<'a>(&'a self) -> NodeRef<'a> {
                NodeRef::new(self.root, &self.arena)
            }
        }

        $vis type Arena = $crate::tree::Arena<$item>;
        $vis type Node = $crate::tree::Node<$item>;
        $vis type NodeRef<'a> = $crate::tree::NodeRef<'a, $item>;
        $vis type NodeMut<'a> = $crate::tree::NodeMut<'a, $item>;
        $vis type NodeId = $crate::tree::NodeId;
    };
}
