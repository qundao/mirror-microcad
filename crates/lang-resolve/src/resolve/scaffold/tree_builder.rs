// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::resolve::ResolveError;
use crate::scaffold::Scaffold;
use crate::{ResolveContext, mir};
use microcad_package::tree::SymbolHandle;

pub struct TreeBuilder {
    pub tree: mir::UnresolvedSymbolTree,
    // Stack of active parents
    pub stack: Vec<SymbolHandle>,
}

impl TreeBuilder {
    pub fn new(root_data: impl Into<mir::UnresolvedSymbolTree>) -> Self {
        Self {
            tree: root_data.into(),
            stack: vec![SymbolHandle::root()],
        }
    }

    /// Add a sub-tree to the current parent
    pub fn add(&mut self, tree: impl Into<mir::UnresolvedSymbolTree>) -> &mut Self {
        let parent = self.stack.last().copied();
        self.tree.insert(parent, tree);
        self
    }

    pub fn scaffold<'a>(
        &mut self,
        context: &mut ResolveContext,
        mut items: impl Iterator<Item = &'a dyn Scaffold>,
    ) -> Result<(), ResolveError> {
        items.try_for_each(|item| {
            self.add(item.scaffold(context)?);
            Ok(())
        })?;
        Ok(())
    }

    /// Enter a child scope (push to stack)
    pub fn enter(&mut self, tree: impl Into<mir::UnresolvedSymbolTree>) -> &mut Self {
        self.stack
            .push(self.tree.last_handle().expect("At least one node"));
        self.add(tree);
        // The last inserted node (the one we just added) becomes the new parent
        self
    }

    /// Exit the current scope (pop from stack)
    pub fn exit(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    pub fn build(self) -> mir::UnresolvedSymbolTree {
        self.tree
    }
}
