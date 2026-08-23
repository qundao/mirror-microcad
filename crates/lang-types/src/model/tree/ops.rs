// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Internal Model tree operations

use crate::model;

impl model::ModelTree {
    /// Static helper to copy a sub-tree from `source_arena` into a target `ModelArena`.
    pub(super) fn adopt_tree_to_arena(
        target_arena: &mut model::Arena,
        source_id: model::NodeId,
        source_arena: &model::Arena,
    ) -> model::NodeId {
        let source_node = model::NodeRef::new(source_id, source_arena);

        // Create matching node in target arena
        let new_id = target_arena.new_node(source_node.get().clone());

        // Traverse children recursively
        for child in source_node.children() {
            let child_id = Self::adopt_tree_to_arena(target_arena, child.id, source_arena);
            new_id.append(child_id, target_arena);
        }

        new_id
    }

    /// Recursive helper to reconstruct the tree, replacing placeholders.
    pub(super) fn replace_placeholders_recursive(
        current_id: model::NodeId,
        source_arena: &model::Arena,
        input_model: &model::ModelTree,
        target_arena: &mut model::Arena,
    ) -> model::NodeId {
        let current_node = model::NodeRef::new(current_id, source_arena);

        // --- MATCH PLACEHOLDER ---
        if current_node.element == model::Element::InputPlaceholder {
            // Replace this node AND its descendants with a full copy of `input_model`
            return Self::adopt_tree_to_arena(target_arena, input_model.root, &input_model.arena);
        }

        // --- REGULAR NODE ---
        // 1. Copy the current node content into the target arena
        let new_id = target_arena.new_node(current_node.get().clone());

        // 2. Process children recursively and attach them
        for child in current_node.children() {
            let new_child_id = Self::replace_placeholders_recursive(
                child.id,
                source_arena,
                input_model,
                target_arena,
            );
            new_id.append(new_child_id, target_arena);
        }

        new_id
    }

    /// Looks up input and output properties in all descendants of the model tree.
    ///
    /// Recursive depth-first helper to search nodes in `self.arena`.
    pub(super) fn get_property_recursive(
        &self,
        current_id: model::NodeId,
        target_id: impl AsRef<str>,
    ) -> Option<&model::Property> {
        let node = model::NodeRef::new(current_id, &self.arena);

        // 1. Check if the current node's model contains a matching Input/Output property
        if let Some(prop) = node.get().get_property(target_id.as_ref())
            && matches!(
                prop.ty,
                model::PropertyType::Input | model::PropertyType::Output
            )
        {
            return Some(prop);
        }

        // 2. Recursively search children
        for child in node.children() {
            if let Some(prop) = self.get_property_recursive(child.id, target_id.as_ref()) {
                return Some(prop);
            }
        }

        None
    }
}
