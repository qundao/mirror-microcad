// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! ModelTree

use serde::{Deserialize, Serialize};

use crate::{Model, ModelRef, Models, Ty, Type, model::ModelContent};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ModelHandle(pub usize);

impl ModelHandle {
    pub fn root() -> Self {
        Self(0)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct ModelTree {
    nodes: Vec<Model>,
}

impl ModelTree {
    /// Create an empty [`SymbolTree`].
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
        }
    }

    /// Helper to access the root directly
    pub fn root(&'_ self) -> Option<ModelRef<'_>> {
        self.get(ModelHandle::root())
    }

    /// Return the handle of the last node in this tree.
    pub fn last_handle(&'_ self) -> Option<ModelHandle> {
        if !self.nodes.is_empty() {
            Some(ModelHandle(self.nodes.len() - 1))
        } else {
            None
        }
    }

    /// Inserts another `ModelTree` into this tree, optionally attaching its roots to `parent`.
    /// Returns the handle of the inserted root node (if `tree` was not empty).
    pub fn insert(
        &mut self,
        parent: Option<ModelHandle>,
        tree: impl Into<ModelTree>,
    ) -> Option<ModelHandle> {
        let tree = tree.into();
        if tree.nodes.is_empty() {
            return None;
        }

        let base_index = self.nodes.len();
        let first_inserted_handle = ModelHandle(base_index);

        // Map every node from the incoming tree with adjusted index handles
        let mapped_nodes = tree.nodes.into_iter().map(|model| Model {
            content: model.content,
            // If the node was a root in the source tree (parent == None), re-parent it to `parent`.
            // Otherwise, offset its existing parent index by base_index.
            parent: match model.parent {
                Some(ModelHandle(p_idx)) => Some(ModelHandle(p_idx + base_index)),
                None => parent,
            },
            children: Models::from_iter(
                model
                    .children
                    .items
                    .into_iter()
                    .map(|ModelHandle(c_idx)| ModelHandle(c_idx + base_index)),
            ),
        });

        self.nodes.extend(mapped_nodes);

        // If a parent handle was specified, register the top-level inserted node into parent's children
        if let Some(parent_handle) = parent
            && let Some(parent_node) = self.get_mut(parent_handle)
        {
            parent_node.children.insert(first_inserted_handle);
        }

        Some(first_inserted_handle)
    }

    pub fn get<'tree>(&'tree self, handle: ModelHandle) -> Option<ModelRef<'tree>> {
        self.nodes
            .get(handle.index())
            .map(|symbol| ModelRef::new(symbol, self, handle))
    }

    fn get_mut(&mut self, handle: ModelHandle) -> Option<&mut Model> {
        self.nodes.get_mut(handle.index())
    }
}

impl Ty for ModelTree {
    fn ty(&self) -> Type {
        self.root().map(|n| n.ty()).unwrap_or(Type::Invalid)
    }
}

impl Default for ModelTree {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Model> for ModelTree {
    fn from(root: Model) -> Self {
        Self { nodes: vec![root] }
    }
}

impl From<ModelContent> for ModelTree {
    fn from(content: ModelContent) -> Self {
        Self::from(Model::from(content))
    }
}

impl std::fmt::Display for ModelTree {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
