// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Index-based tree node reference helpers.

pub use crate::tree::{Arena, NodeId};

/// A convenience wrapper pairing a `NodeId` with a borrowed `Arena`.
#[derive(Clone, Copy)]
pub struct NodeRef<'a, T> {
    pub id: NodeId,
    pub arena: &'a Arena<T>,
}

impl<'a, T> NodeRef<'a, T> {
    pub fn new(id: NodeId, arena: &'a Arena<T>) -> Self {
        Self { id, arena }
    }

    /// Access the underlying value payload.
    pub fn get(&self) -> &'a T {
        self.arena[self.id].get()
    }

    /// Get the parent node as a `NodeRef`.
    pub fn parent(&self) -> Option<NodeRef<'a, T>> {
        self.arena[self.id]
            .parent()
            .map(|parent_id| NodeRef::new(parent_id, self.arena))
    }

    /// Iterate over children wrapped as `NodeRef`s.
    pub fn children(&self) -> impl Iterator<Item = NodeRef<'a, T>> {
        let arena = self.arena;
        self.id
            .children(arena)
            .map(move |id| NodeRef::new(id, arena))
    }

    /// Iterate over ancestors wrapped as `NodeRef`s.
    pub fn ancestors(&self) -> impl Iterator<Item = NodeRef<'a, T>> {
        let arena = self.arena;
        self.id
            .ancestors(arena)
            .map(move |id| NodeRef::new(id, arena))
    }
}

// Deref into the underlying payload for zero-cost value access
impl<'a, T> std::ops::Deref for NodeRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

pub struct NodeMut<'a, T> {
    pub id: NodeId,
    pub arena: &'a mut Arena<T>,
}

impl<'a, T> NodeMut<'a, T> {
    pub fn get_mut(&mut self) -> &mut T {
        self.arena[self.id].get_mut()
    }

    pub fn append(&mut self, child: NodeId) {
        self.id.append(child, self.arena);
    }
}
