// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_types::{ModelNodeRef, ModelTree};

use crate::RenderOutput;

/// A model tree with a root node.
#[derive(Debug, Clone)]
pub struct RenderTree {
    pub root: RenderNodeId,
    pub arena: RenderArena,
}

impl RenderTree {
    pub fn new(model_tree: &ModelTree) -> Self {
        let mut arena = RenderArena::new();

        RenderTree {
            root: Self::build_node(model_tree.root(), &mut arena),
            arena,
        }
    }

    /// Helper method to recursively transform ModelNodeRef into RenderNodeId
    fn build_node(model_node: ModelNodeRef<'_>, arena: &mut RenderArena) -> RenderNodeId {
        // 1. Transform the ModelOutput / Model data into a RenderOutput
        let render_output = RenderOutput::new(model_node);

        // 2. Insert the new node into the RenderArena
        let current_node_id = arena.new_node(render_output);

        // 3. Traverse and append all children recursively
        for child_ref in model_node.children() {
            let child_node_id = Self::build_node(child_ref, arena);
            current_node_id.append(child_node_id, arena);
        }

        current_node_id
    }

    pub fn root<'a>(&'a self) -> RenderNodeRef<'a> {
        RenderNodeRef::new(self.root, &self.arena)
    }
}

pub type RenderArena = microcad_lang_base::tree::Arena<RenderOutput>;
pub type RenderNode = microcad_lang_base::tree::Node<RenderOutput>;
pub type RenderNodeRef<'a> = microcad_lang_base::tree::NodeRef<'a, RenderOutput>;
pub type RenderNodeMut<'a> = microcad_lang_base::tree::NodeMut<'a, RenderOutput>;
pub type RenderNodeId = microcad_lang_base::tree::NodeId;
