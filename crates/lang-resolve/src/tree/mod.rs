// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad symbol tree.
//!
//! This is an arena-based intended to transform IR nodes into symbols.
//! The symbols have an unresolved and a resolved variant, hence this tree is generic.

use serde::{Deserialize, Serialize};

pub mod iterators;
pub mod symbol;

pub use symbol::*;

/// A generic symbol tree.
#[derive(Debug, PartialEq, Hash, Default, Serialize, Deserialize)]
#[serde(bound(serialize = "DEF: Serialize", deserialize = "DEF: Deserialize<'de>"))]
pub struct SymbolTree<DEF: Serialize> {
    nodes: Vec<Symbol<DEF>>,
}

impl<DEF: Serialize> SymbolTree<DEF> {
    /// Create an empty [`SymbolTree`].
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
        }
    }

    /// Helper to access the root directly
    pub fn root(&'_ self) -> Option<SymbolRef<'_, DEF>> {
        self.get(SymbolHandle(0))
    }

    /// Return the handle of the last node in this tree.
    pub fn last_handle(&'_ self) -> Option<SymbolHandle> {
        if !self.nodes.is_empty() {
            Some(SymbolHandle(self.nodes.len() - 1))
        } else {
            None
        }
    }

    pub fn insert(&mut self, parent: Option<SymbolHandle>, tree: impl Into<SymbolTree<DEF>>) {
        let tree = tree.into();

        let base_index = self.nodes.len();
        let handle = SymbolHandle(base_index);

        let nodes = tree.nodes.into_iter().map(|node| Symbol {
            meta_data: node.meta_data,
            def: node.def,
            parent: node
                .parent
                .map(|SymbolHandle(index)| SymbolHandle(index + base_index))
                .or(parent),
            children: SymbolIndex::from_iter(
                node.children
                    .items
                    .into_iter()
                    .map(|SymbolHandle(index)| SymbolHandle(index + base_index)),
            ),
        });
        self.nodes.extend(nodes.into_iter());

        if let Some(parent) = parent {
            if let Some(parent_node) = self.get_mut(parent) {
                parent_node.children.insert(handle);
            }
        }
    }

    pub fn get<'tree>(&'tree self, handle: SymbolHandle) -> Option<SymbolRef<'tree, DEF>> {
        self.nodes
            .get(handle.index())
            .map(|symbol| SymbolRef::new(symbol, self, handle))
    }

    fn get_mut(&mut self, handle: SymbolHandle) -> Option<&mut Symbol<DEF>> {
        self.nodes.get_mut(handle.index())
    }
}

impl<DEF: Serialize> From<Symbol<DEF>> for SymbolTree<DEF> {
    fn from(root: Symbol<DEF>) -> Self {
        Self { nodes: vec![root] }
    }
}
