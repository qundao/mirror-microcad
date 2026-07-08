// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad symbol tree.

mod data;
pub mod def;
mod iterators;
mod storage;

use std::hash::Hash;

use derive_more::Deref;
use microcad_lang_base::{
    ComputedHash, HashId, HashMap, Hashed, Id, Identifier, SrcRef, Version, element::Visibility,
};

pub use storage::TreeStorage;

pub use iterators::*;

use data::*;

use def::SymbolDef;
use serde::{Deserialize, Serialize};

pub use data::{SymbolAttributes, SymbolData};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolHandle(microcad_lang_base::HashId);

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SymbolIndex {
    items: Vec<SymbolHandle>,
}

impl SymbolIndex {
    pub fn insert(&mut self, hash: SymbolHandle) {
        self.items.push(hash);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&SymbolHandle> {
        self.items.get(index)
    }

    pub fn get_by_id<'tree>(&self, tree: &'tree SymbolTree, id: Id) -> Option<SymbolRef<'tree>> {
        self.items
            .iter()
            .filter_map(|hash| tree.get(*hash))
            .find(|symbol_ref| &symbol_ref.data.id.id() == &id)
    }

    pub fn refs<'tree>(&self, tree: &'tree SymbolTree) -> impl Iterator<Item = SymbolRef<'tree>> {
        self.items.iter().filter_map(|hash| tree.get(*hash))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]

pub struct Symbol {
    data: SymbolData,
    handle: SymbolHandle,
    parent: Option<SymbolHandle>,
    children: SymbolIndex,
}

pub struct SymbolPath(Vec<Id>);

#[derive(Debug, Deref, Clone, Copy)]
pub struct SymbolRef<'tree> {
    #[deref]
    symbol: &'tree Symbol,
    tree: &'tree SymbolTree,
}

impl<'tree> SymbolRef<'tree> {
    pub fn id(&self) -> &Id {
        self.data.id.id()
    }

    pub fn symbol(&self) -> &'tree Symbol {
        self.symbol
    }

    pub fn tree(&'tree self) -> &'tree SymbolTree {
        self.tree
    }

    pub fn children(&self) -> Children<'tree> {
        Children::new(*self)
    }

    pub fn descendants(&self) -> Descendants<'tree> {
        Descendants::new(*self)
    }

    pub fn handle(&self) -> SymbolHandle {
        self.handle
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SymbolTreeMetadata {
    version: Version,
}

impl Default for SymbolTreeMetadata {
    fn default() -> Self {
        Self {
            version: Version::current(),
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SymbolTree {
    metadata: SymbolTreeMetadata,
    nodes: HashMap<SymbolHandle, Symbol>,
    root: Option<SymbolHandle>,
}

impl SymbolTree {
    pub fn new() -> Self {
        Self::default()
    }

    // Helper to access the root directly
    pub fn root(&'_ self) -> Option<SymbolRef<'_>> {
        self.root.and_then(|hash| self.get(hash))
    }

    pub fn insert(&'_ mut self, parent: Option<SymbolHandle>, data: impl Into<SymbolData>) {
        let data = data.into();
        let handle = data.get_handle();

        // If there is no parent, this is the root
        if parent.is_none() {
            assert!(self.root.is_none());
            self.root = Some(handle);
        }

        self.nodes.insert(
            handle,
            Symbol {
                data,
                handle,
                parent,
                children: SymbolIndex::default(),
            },
        );

        if let Some(parent) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&parent) {
                parent_node.children.insert(handle);
            }
        }
    }

    pub fn get(&'_ self, handle: SymbolHandle) -> Option<SymbolRef<'_>> {
        self.nodes.get(&handle).map(|symbol| SymbolRef {
            symbol,
            tree: &self,
        })
    }
}

pub struct SymbolTreeBuilder {
    tree: SymbolTree,
    // Stack of active parents
    stack: Vec<SymbolHandle>,
}

impl SymbolTreeBuilder {
    pub fn new(root_data: impl Into<SymbolData>) -> Self {
        let mut tree = SymbolTree::new();
        let root_data = root_data.into();
        let root_handle = SymbolHandle(root_data.computed_hash());

        tree.insert(None, root_data);

        Self {
            tree,
            stack: vec![root_handle],
        }
    }

    /// Add a child to the current parent
    pub fn add(&mut self, data: impl Into<SymbolData>) -> &mut Self {
        let parent = self.stack.last().copied();
        let data = data.into();

        self.tree.insert(parent, data);
        self
    }

    /// Enter a child scope (push to stack)
    pub fn enter(&mut self, data: impl Into<SymbolData>) -> &mut Self {
        let data = data.into();
        let handle = data.get_handle();
        self.add(data);
        // The last inserted node (the one we just added) becomes the new parent
        self.stack.push(handle);
        self
    }

    /// Exit the current scope (pop from stack)
    pub fn exit(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    pub fn build(self) -> SymbolTree {
        self.tree
    }
}
