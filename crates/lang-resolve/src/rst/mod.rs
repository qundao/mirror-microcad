// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad resolved symbol tree (RST).

mod data;
pub mod def;
mod iterators;

use std::hash::Hash;

use derive_more::{Deref, From};
use microcad_lang_base::{
    ComputedHash, HashId, HashMap, Hashed, Id, Identifier, SrcRef, Version, element::Visibility,
};

pub use iterators::*;

use data::*;

use def::SymbolDef;
use microcad_lang_proc_macros::Artifact;
use serde::{Deserialize, Serialize};

pub use data::{SymbolAttributes, SymbolData};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolHandle(usize);

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolDataHandle(microcad_lang_base::HashId);

#[derive(Debug, Default, Hash, PartialEq, Serialize, Deserialize)]
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

    pub fn get_by_id<'rst>(&self, tree: &'rst Rst, id: Id) -> Option<SymbolRef<'rst>> {
        self.items
            .iter()
            .filter_map(|hash| tree.get(*hash))
            .find(|symbol_ref| &symbol_ref.data.id.id() == &id)
    }

    pub fn refs<'rst>(&self, tree: &'rst Rst) -> impl Iterator<Item = SymbolRef<'rst>> {
        self.items.iter().filter_map(|hash| tree.get(*hash))
    }
}

impl FromIterator<SymbolHandle> for SymbolIndex {
    fn from_iter<T: IntoIterator<Item = SymbolHandle>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]

pub struct Symbol {
    data: SymbolDataHandle,
    parent: Option<SymbolHandle>,
    children: SymbolIndex,
}

impl Symbol {
    pub fn new(data: SymbolDataHandle) -> Self {
        Self {
            data,
            parent: None,
            children: Default::default(),
        }
    }
}

#[derive(Debug, Clone, From)]
pub struct SymbolPath(Vec<Id>);

impl From<&str> for SymbolPath {
    fn from(s: &str) -> Self {
        Self(
            s.split("::")
                .filter(|segment| !segment.is_empty())
                .map(|segment| Id::from(segment))
                .collect(),
        )
    }
}

#[derive(Debug, Deref, Clone, Copy)]
pub struct SymbolRef<'rst> {
    #[deref]
    symbol: &'rst Symbol,
    data: &'rst SymbolData,
    rst: &'rst Rst,
    handle: SymbolHandle,
}

impl<'rst> SymbolRef<'rst> {
    pub fn id(&self) -> &Id {
        self.data.id.id()
    }

    pub fn symbol(&self) -> &'rst Symbol {
        self.symbol
    }

    pub fn tree(&self) -> &'rst Rst {
        self.rst
    }

    pub fn handle(&self) -> SymbolHandle {
        self.handle
    }

    pub fn children(&self) -> Children<'rst> {
        Children::new(*self)
    }

    pub fn descendants(&self) -> Descendants<'rst> {
        Descendants::new(*self)
    }

    pub fn search_down(&self, path: impl Into<SymbolPath>) -> Vec<SymbolRef<'rst>> {
        let path = path.into();
        match path.0.as_slice() {
            // Base case: path is empty, return this node
            [] => vec![*self],

            // Recursive case: match first ID, then search descendants
            [first, rest @ ..] => {
                self.children()
                    .filter(|child| child.id() == first)
                    .flat_map(|child| {
                        // If there is more path, continue searching
                        if rest.is_empty() {
                            vec![child]
                        } else {
                            child.search_down(rest.to_vec())
                        }
                    })
                    .collect()
            }
        }
    }

    /// Resolves a path relative to the current symbol.
    /// If path starts with root indicator, it searches from top.
    /// Otherwise, it performs a local-outward search (upward).
    pub fn resolve(&self, path: impl Into<SymbolPath>) -> Option<SymbolRef<'rst>> {
        // 1. If searching from current node, look upward for the first component
        let path = path.into();
        let mut current = *self;

        loop {
            // Check if current node matches the first element of the path
            if current.id() == path.0.first()? {
                // If the path matches, descend into children to find the rest
                if let Some(target) = self.descend(&current, &path.0[1..]) {
                    return Some(target);
                }
            }

            // Move up
            match current.parent {
                Some(parent_handle) => current = self.rst.get(parent_handle)?,
                None => break, // Reached root
            }
        }
        None
    }

    /// Helper to descend into children
    fn descend(&self, node: &SymbolRef<'rst>, remaining_path: &[Id]) -> Option<SymbolRef<'rst>> {
        if remaining_path.is_empty() {
            return Some(*node);
        }

        node.children()
            .filter(|child| child.id() == &remaining_path[0])
            .find_map(|child| self.descend(&child, &remaining_path[1..]))
    }
}

/// The resolved symbol tree (RST).
#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Artifact)]
pub struct Rst {
    nodes: Vec<Symbol>,
    data: HashMap<SymbolDataHandle, SymbolData>,
}

impl Rst {
    pub fn new() -> Self {
        Self::default()
    }

    // Helper to access the root directly
    pub fn root(&'_ self) -> Option<SymbolRef<'_>> {
        self.get(SymbolHandle(0))
    }

    fn insert_data(&mut self, data: impl Into<SymbolData>) -> SymbolDataHandle {
        let data = data.into();
        let handle = data.get_handle();
        self.data.insert(handle, data);
        handle
    }

    pub fn insert_node(
        &'_ mut self,
        parent: Option<SymbolHandle>,
        data: impl Into<SymbolData>,
    ) -> SymbolHandle {
        let data = self.insert_data(data);

        let symbol = Symbol {
            data,
            parent,
            children: SymbolIndex::default(),
        };

        let handle = SymbolHandle(self.nodes.len());
        self.nodes.push(symbol);

        if let Some(parent) = parent {
            if let Some(parent_node) = self.get_mut(parent) {
                parent_node.children.insert(handle);
            }
        }
        handle
    }

    pub fn insert_tree(&mut self, parent: Option<SymbolHandle>, rst: impl Into<Rst>) {
        let rst = rst.into();
        self.data.extend(rst.data.into_iter());

        let base_index = self.nodes.len();
        let handle = SymbolHandle(base_index);

        let nodes = rst.nodes.into_iter().map(|node| Symbol {
            data: node.data,
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

    fn get_data(&self, handle: SymbolDataHandle) -> Option<&SymbolData> {
        self.data.get(&handle)
    }

    pub fn get(&'_ self, handle: SymbolHandle) -> Option<SymbolRef<'_>> {
        self.nodes.get(handle.0).map(|symbol| SymbolRef {
            data: self.get_data(symbol.data).unwrap(),
            symbol,
            rst: &self,
            handle,
        })
    }

    fn get_mut(&mut self, handle: SymbolHandle) -> Option<&mut Symbol> {
        self.nodes.get_mut(handle.0)
    }
}

impl From<SymbolData> for Rst {
    fn from(data: SymbolData) -> Self {
        let data_handle = data.get_handle();
        Rst {
            nodes: vec![Symbol::new(data_handle)],
            data: [(data_handle, data)].into_iter().collect(),
        }
    }
}

pub struct Builder {
    tree: Rst,
    // Stack of active parents
    stack: Vec<SymbolHandle>,
}

impl Builder {
    pub fn new(root_data: impl Into<Rst>) -> Self {
        Self {
            tree: root_data.into(),
            stack: vec![SymbolHandle(0)],
        }
    }

    /// Add a child to the current parent
    pub fn add_node(&mut self, data: impl Into<SymbolData>) -> &mut Self {
        let parent = self.stack.last().copied();
        self.tree.insert_node(parent, data);
        self
    }

    /// Add a sub-tree to the current parent
    pub fn add_tree(&mut self, rst: Rst) -> &mut Self {
        let parent = self.stack.last().copied();
        self.tree.insert_tree(parent, rst);
        self
    }

    /// Enter a child scope (push to stack)
    pub fn enter(&mut self, data: impl Into<SymbolData>) -> &mut Self {
        self.stack.push(SymbolHandle(self.tree.nodes.len() - 1));
        self.add_node(data);
        // The last inserted node (the one we just added) becomes the new parent
        self
    }

    /// Exit the current scope (pop from stack)
    pub fn exit(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    pub fn build(self) -> Rst {
        self.tree
    }
}
