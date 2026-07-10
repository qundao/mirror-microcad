// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad resolved symbol tree (RST).

mod data;
pub mod def;
mod iterators;

use std::{collections::BTreeMap, hash::Hash};

use derive_more::{Deref, From};
use microcad_lang_base::{
    Artifact, ComputedHash, HashId, HashMap, Hashed, Id, Identifier, SrcRef, Version,
    element::Visibility,
};

pub use iterators::*;

use data::*;

use def::SymbolDef;
use microcad_lang_lower::ir::QualifiedName;
use microcad_lang_proc_macros::Artifact;
use serde::{Deserialize, Serialize};

pub use data::{SymbolAttributes, SymbolData};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolHandle(usize);

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

    pub fn get_by_id<'rst, DATA: Serialize>(
        &self,
        tree: &'rst SymbolTree<DATA>,
        id: Id,
    ) -> Option<SymbolRef<'rst, DATA>> {
        self.items
            .iter()
            .filter_map(|hash| tree.get(*hash))
            .find(|symbol_ref| symbol_ref.id() == &id)
    }

    pub fn refs<'rst, DATA: Serialize>(
        &self,
        tree: &'rst SymbolTree<DATA>,
    ) -> impl Iterator<Item = SymbolRef<'rst, DATA>> {
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
    pub id: Id,
    data: SymbolDataHandle,
    parent: Option<SymbolHandle>,
    children: SymbolIndex,
}

impl Symbol {
    pub fn new(id: Id, data: SymbolDataHandle) -> Self {
        Self {
            id,
            data,
            parent: None,
            children: Default::default(),
        }
    }
}

#[derive(Debug, Clone, From, Hash, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy)]
pub struct SymbolRef<'rst, DATA: Serialize> {
    symbol: &'rst Symbol,
    data: &'rst DATA,
    tree: &'rst SymbolTree<DATA>,
    handle: SymbolHandle,
}

impl<'rst, DATA: Serialize> std::ops::Deref for SymbolRef<'rst, DATA> {
    type Target = Symbol;

    fn deref(&self) -> &Self::Target {
        self.symbol
    }
}

impl<'rst, DATA: Serialize> SymbolRef<'rst, DATA> {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn symbol(&self) -> &'rst Symbol {
        self.symbol
    }

    pub fn tree(&self) -> &'rst SymbolTree<DATA> {
        self.tree
    }

    pub fn handle(&self) -> SymbolHandle {
        self.handle
    }

    pub fn children(&self) -> Children<'rst, DATA>
    where
        DATA: Clone,
    {
        Children::new(self.clone())
    }

    pub fn descendants(&self) -> Descendants<'rst, DATA>
    where
        DATA: Clone,
    {
        Descendants::new(self.clone())
    }

    pub fn search_down(&self, path: impl Into<SymbolPath>) -> Vec<SymbolRef<'rst, DATA>>
    where
        DATA: Clone,
    {
        let path = path.into();
        match path.0.as_slice() {
            // Base case: path is empty, return this node
            [] => vec![self.clone()],

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
    pub fn resolve(&self, path: impl Into<SymbolPath>) -> Option<SymbolRef<'rst, DATA>>
    where
        DATA: Clone,
    {
        // 1. If searching from current node, look upward for the first component
        let path = path.into();
        let mut current = self.clone();

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
                Some(parent_handle) => current = self.tree.get(parent_handle)?,
                None => break, // Reached root
            }
        }
        None
    }

    /// Helper to descend into children
    fn descend(
        &self,
        node: &SymbolRef<'rst, DATA>,
        remaining_path: &[Id],
    ) -> Option<SymbolRef<'rst, DATA>>
    where
        DATA: Clone,
    {
        if remaining_path.is_empty() {
            return Some(node.clone());
        }

        node.children()
            .filter(|child| child.id() == &remaining_path[0])
            .find_map(|child| self.descend(&child, &remaining_path[1..]))
    }
}

/// The resolved symbol tree (RST).
#[derive(Debug, PartialEq, Hash, Default, Serialize, Deserialize)]
#[serde(bound(serialize = "DATA: Serialize", deserialize = "DATA: Deserialize<'de>"))]

pub struct SymbolTree<DATA: Serialize> {
    nodes: Vec<Symbol>,
    data: BTreeMap<SymbolDataHandle, DATA>,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct UnresolvedName(SymbolPath);

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResolvedName {
    Local(Id),
    Symbol(SymbolHandle),
    Unresolved(SymbolPath),
}

pub type ResolvedSymbolData = SymbolData<ResolvedName>;

pub type Rst = SymbolTree<ResolvedSymbolData>;

impl<DATA: Serialize> SymbolTree<DATA> {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            data: Default::default(),
        }
    }

    // Helper to access the root directly
    pub fn root(&'_ self) -> Option<SymbolRef<'_, DATA>> {
        self.get(SymbolHandle(0))
    }

    pub fn insert(&mut self, parent: Option<SymbolHandle>, tree: impl Into<SymbolTree<DATA>>) {
        let rst = tree.into();
        self.data.extend(rst.data.into_iter());

        let base_index = self.nodes.len();
        let handle = SymbolHandle(base_index);

        let nodes = rst.nodes.into_iter().map(|node| Symbol {
            id: node.id,
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

    fn get_data(&self, handle: SymbolDataHandle) -> Option<&DATA> {
        self.data.get(&handle)
    }

    pub fn get(&'_ self, handle: SymbolHandle) -> Option<SymbolRef<'_, DATA>> {
        self.nodes.get(handle.0).map(|symbol| SymbolRef {
            data: self.get_data(symbol.data).unwrap(),
            symbol,
            tree: &self,
            handle,
        })
    }

    fn get_mut(&mut self, handle: SymbolHandle) -> Option<&mut Symbol> {
        self.nodes.get_mut(handle.0)
    }
}

impl<NAME: Serialize + Hash> From<(Id, SymbolData<NAME>)> for SymbolTree<SymbolData<NAME>> {
    fn from(data: (Id, SymbolData<NAME>)) -> Self {
        let id = data.0;
        let data_handle = data.1.get_handle();
        Self {
            nodes: vec![Symbol::new(id, data_handle)],
            data: [(data_handle, data.1)].into_iter().collect(),
        }
    }
}

pub struct Builder {
    tree: SymbolTree<SymbolData<UnresolvedName>>,
    // Stack of active parents
    stack: Vec<SymbolHandle>,
}

impl Builder {
    pub fn new(root_data: impl Into<SymbolTree<SymbolData<UnresolvedName>>>) -> Self {
        Self {
            tree: root_data.into(),
            stack: vec![SymbolHandle(0)],
        }
    }

    /// Add a sub-tree to the current parent
    pub fn add(&mut self, tree: impl Into<SymbolTree<SymbolData<UnresolvedName>>>) -> &mut Self {
        let parent = self.stack.last().copied();
        self.tree.insert(parent, tree);
        self
    }

    /// Enter a child scope (push to stack)
    pub fn enter(&mut self, tree: impl Into<SymbolTree<SymbolData<UnresolvedName>>>) -> &mut Self {
        self.stack.push(SymbolHandle(self.tree.nodes.len() - 1));
        self.add(tree);
        // The last inserted node (the one we just added) becomes the new parent
        self
    }

    /// Exit the current scope (pop from stack)
    pub fn exit(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    pub fn build(self) -> SymbolTree<SymbolData<UnresolvedName>> {
        self.tree
    }
}
