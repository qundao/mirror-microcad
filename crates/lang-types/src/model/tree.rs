// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree.

use microcad_lang_proc_macros::Artifact;
use serde::{Deserialize, Serialize};

use crate::{
    Model, Ty, Type,
    model::{self, NodeExt, element::BuiltinWorkpiece},
};

/// A model tree with a root node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Artifact)]
pub struct ModelTree {
    pub root: model::NodeId,
    pub arena: model::Arena,
}

impl ModelTree {
    /// Combines `self` and `other` under a boolean operation (`Union`, `Intersection`, or `Difference`).
    ///
    /// If `self`'s root element is already a matching `BooleanOp`, it reuses `self`'s root
    /// and appends `other` to the inner `Group` child (flattening equal adjacent operations).
    /// Otherwise, it builds a new `ModelTree` with the structure:
    ///
    /// Root: ModelNode(BooleanOp)
    /// └── Group
    ///     ├── lhs (self)
    ///     └── rhs (other)
    pub fn apply_boolean_op(mut self, op: model::BooleanOp, rhs: Self) -> Self {
        // Check if `self`'s root is already a BooleanOp with the SAME operation
        if let model::Element::BuiltinWorkpiece(BuiltinWorkpiece::BooleanOp(current_op)) =
            self.root().element
            && current_op == op
        {
            // Find the single child Group under the current root
            if let Some(group_ref) = self.root().into_group_child() {
                let group_id = group_ref.id;

                // Copy rhs's root and subtree into self's arena under the group
                let rhs_root_id = self.adopt_tree(rhs.root, &rhs.arena);
                group_id.append(rhs_root_id, &mut self.arena);

                return self;
            }
        }

        // --- Otherwise, construct the new tree structure ---
        let mut arena = model::Arena::new();

        // 1. Create the top-level BooleanOp root node
        let root_model = Model {
            element: model::Element::BuiltinWorkpiece(BuiltinWorkpiece::BooleanOp(op)),
            ..Model::default()
        };
        let root = arena.new_node(root_model);

        // 2. Create the child Group node
        let group_model = Model {
            element: model::Element::Group,
            ..Model::default()
        };
        let group_id = arena.new_node(group_model);
        root.append(group_id, &mut arena);

        // 3. Adopt both self's root and rhs's root into the new arena under Group
        let lhs_root_id = Self::adopt_tree_to_arena(&mut arena, self.root, &self.arena);
        let rhs_root_id = Self::adopt_tree_to_arena(&mut arena, rhs.root, &rhs.arena);

        group_id.append(lhs_root_id, &mut arena);
        group_id.append(rhs_root_id, &mut arena);

        Self { root, arena }
    }

    /// Recursively copies a sub-tree from `source_arena` into `self.arena`.
    pub fn adopt_tree(
        &mut self,
        source_id: model::NodeId,
        source_arena: &model::Arena,
    ) -> model::NodeId {
        Self::adopt_tree_to_arena(&mut self.arena, source_id, source_arena)
    }

    /// Static helper to copy a sub-tree from `source_arena` into a target `ModelArena`.
    fn adopt_tree_to_arena(
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

    /// Returns a new `ModelTree` where every `Element::InputPlaceholder` node
    /// (and its descendants) is replaced with a deep copy of `input_model`.
    pub fn replace_input_placeholders(&self, input_model: &ModelTree) -> Self {
        let mut new_arena = model::Arena::new();

        // Recursively build the transformed tree starting from root
        let new_root = Self::replace_placeholders_recursive(
            self.root,
            &self.arena,
            input_model,
            &mut new_arena,
        );

        Self {
            root: new_root,
            arena: new_arena,
        }
    }

    /// Recursive helper to reconstruct the tree, replacing placeholders.
    fn replace_placeholders_recursive(
        current_id: model::NodeId,
        source_arena: &model::Arena,
        input_model: &ModelTree,
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
}

impl std::hash::Hash for ModelTree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.root().hash(state);
    }
}

impl std::fmt::Display for ModelTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root().fmt(f)
    }
}

impl Ty for ModelTree {
    fn ty(&self) -> Type {
        Type::Model(self.root().output_type())
    }
}

/// `lhs | rhs` -> Union operation
impl std::ops::BitOr for ModelTree {
    type Output = ModelTree;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.apply_boolean_op(model::BooleanOp::Union, rhs)
    }
}

/// `lhs & rhs` -> Intersection operation
impl std::ops::BitAnd for ModelTree {
    type Output = ModelTree;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.apply_boolean_op(model::BooleanOp::Intersect, rhs)
    }
}

/// `lhs - rhs` -> Difference operation
impl std::ops::Sub for ModelTree {
    type Output = ModelTree;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply_boolean_op(model::BooleanOp::Difference, rhs)
    }
}
