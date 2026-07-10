// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad resolved symbol tree (RST).

mod data;
pub mod def;
mod iterators;

use std::hash::Hash;

use derive_more::From;
use microcad_lang_base::Id;

pub use iterators::*;

use serde::{Deserialize, Serialize};

pub use data::{SymbolAttributes, SymbolData};

use crate::rst::def::{ResolvedSymbolDef, UnresolvedSymbolDef};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolHandle(usize);

#[derive(Debug, Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
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

    pub fn get_by_id<'tree, DEF: Serialize>(
        &self,
        tree: &'tree SymbolTree<DEF>,
        id: Id,
    ) -> Option<SymbolRef<'tree, DEF>> {
        self.items
            .iter()
            .filter_map(|hash| tree.get(*hash))
            .find(|symbol_ref| symbol_ref.id() == &id)
    }

    pub fn refs<'tree, DATA: Serialize>(
        &self,
        tree: &'tree SymbolTree<DATA>,
    ) -> impl Iterator<Item = SymbolRef<'tree, DATA>> {
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

pub struct Symbol<DEF: Serialize> {
    pub data: SymbolData,
    pub def: DEF,
    pub parent: Option<SymbolHandle>,
    pub children: SymbolIndex,
}

impl<DEF: Serialize> Symbol<DEF> {
    pub fn new(data: SymbolData, def: DEF) -> Self {
        Self {
            data,
            def,
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

#[derive(Debug)]
pub struct SymbolRef<'tree, DEF: Serialize>
where
    DEF: 'tree,
{
    symbol: &'tree Symbol<DEF>,
    tree: &'tree SymbolTree<DEF>,
    handle: SymbolHandle,
}

impl<'tree, DEF: Serialize> Clone for SymbolRef<'tree, DEF> {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol.clone(),
            tree: self.tree.clone(),
            handle: self.handle.clone(),
        }
    }
}

impl<'tree, DEF: Serialize> Copy for SymbolRef<'tree, DEF> {}

impl<'tree, DEF: Serialize> std::ops::Deref for SymbolRef<'tree, DEF> {
    type Target = Symbol<DEF>;

    fn deref(&self) -> &Self::Target {
        self.symbol
    }
}

impl<'tree, DEF: Serialize> SymbolRef<'tree, DEF> {
    pub fn id(&self) -> &Id {
        &self.symbol.data.id
    }

    pub fn symbol(&self) -> &'tree Symbol<DEF> {
        self.symbol
    }

    pub fn data(&self) -> &'tree SymbolData {
        &self.symbol.data
    }

    pub fn tree(&self) -> &'tree SymbolTree<DEF> {
        self.tree
    }

    pub fn handle(&self) -> SymbolHandle {
        self.handle
    }

    pub fn children(&self) -> Children<'tree, DEF> {
        Children::new(*self)
    }

    pub fn descendants(&self) -> Descendants<'tree, DEF> {
        Descendants::new(*self)
    }

    pub fn search_down(&self, path: impl Into<SymbolPath>) -> Vec<SymbolRef<'tree, DEF>> {
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
    pub fn resolve(&self, path: impl Into<SymbolPath>) -> Option<SymbolRef<'tree, DEF>> {
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
                Some(parent_handle) => current = self.tree.get(parent_handle)?,
                None => break, // Reached root
            }
        }
        None
    }

    /// Helper to descend into children
    fn descend(
        &self,
        node: &SymbolRef<'tree, DEF>,
        remaining_path: &[Id],
    ) -> Option<SymbolRef<'tree, DEF>> {
        if remaining_path.is_empty() {
            return Some(*node);
        }

        node.children()
            .filter(|child| child.id() == &remaining_path[0])
            .find_map(|child| self.descend(&child, &remaining_path[1..]))
    }
}

/// The resolved symbol tree (RST).
#[derive(Debug, PartialEq, Hash, Default, Serialize, Deserialize)]
#[serde(bound(serialize = "DEF: Serialize", deserialize = "DEF: Deserialize<'de>"))]

pub struct SymbolTree<DEF: Serialize> {
    nodes: Vec<Symbol<DEF>>,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct UnresolvedName(SymbolPath);

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResolvedName {
    Local(Id),
    Symbol(SymbolHandle),
    Unresolved(SymbolPath),
}

pub type Rst = SymbolTree<ResolvedSymbolDef>;

impl<DEF: Serialize> SymbolTree<DEF> {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
        }
    }

    // Helper to access the root directly
    pub fn root(&'_ self) -> Option<SymbolRef<'_, DEF>> {
        self.get(SymbolHandle(0))
    }

    pub fn insert(&mut self, parent: Option<SymbolHandle>, tree: impl Into<SymbolTree<DEF>>) {
        let tree = tree.into();

        let base_index = self.nodes.len();
        let handle = SymbolHandle(base_index);

        let nodes = tree.nodes.into_iter().map(|node| Symbol {
            data: node.data,
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
        self.nodes.get(handle.0).map(|symbol| SymbolRef {
            symbol,
            tree: &self,
            handle,
        })
    }

    fn get_mut(&mut self, handle: SymbolHandle) -> Option<&mut Symbol<DEF>> {
        self.nodes.get_mut(handle.0)
    }
}

pub struct Builder {
    pub tree: SymbolTree<UnresolvedSymbolDef>,
    // Stack of active parents
    pub stack: Vec<SymbolHandle>,
}

impl Builder {
    pub fn new(root_data: impl Into<SymbolTree<UnresolvedSymbolDef>>) -> Self {
        Self {
            tree: root_data.into(),
            stack: vec![SymbolHandle(0)],
        }
    }

    /// Add a sub-tree to the current parent
    pub fn add(&mut self, tree: impl Into<SymbolTree<UnresolvedSymbolDef>>) -> &mut Self {
        let parent = self.stack.last().copied();
        self.tree.insert(parent, tree);
        self
    }

    /// Enter a child scope (push to stack)
    pub fn enter(&mut self, tree: impl Into<SymbolTree<UnresolvedSymbolDef>>) -> &mut Self {
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

    pub fn build(self) -> Rst {
        let root = self.tree.root().expect("Root node expected");

        // Convert unresolved symbols into resolved symbols
        let nodes: Vec<Symbol<ResolvedSymbolDef>> = root
            .descendants()
            .map(|symbol| def::resolve_symbol(symbol).expect("TODO Error handling"))
            .collect();

        Rst { nodes }
    }
}

impl<DEF: Serialize> From<Symbol<DEF>> for SymbolTree<DEF> {
    fn from(root: Symbol<DEF>) -> Self {
        Self { nodes: vec![root] }
    }
}
