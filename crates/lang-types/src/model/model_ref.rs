// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Reference to a node in the model tree.

use derive_more::Deref;
use microcad_lang_base::TreeDisplay;

use crate::{
    Model, ModelTree, Ty, Type,
    model::{Element, ModelHandle, ModelOutputType, iter},
};

#[derive(Debug, Deref, Clone, Copy)]
pub struct ModelRef<'tree> {
    #[deref]
    model: &'tree Model,
    tree: &'tree ModelTree,
    handle: ModelHandle,
}

impl<'tree> ModelRef<'tree> {
    pub fn new(model: &'tree Model, tree: &'tree ModelTree, handle: ModelHandle) -> Self {
        Self {
            model,
            tree,
            handle,
        }
    }

    pub fn model(&self) -> &'tree Model {
        self.model
    }

    pub fn tree(&self) -> &'tree ModelTree {
        self.tree
    }

    pub fn handle(&self) -> ModelHandle {
        self.handle
    }

    pub fn element(&self) -> &'tree Element {
        &self.model.content.element
    }

    /// Return the parent of this model.
    pub fn parent(&self) -> Option<ModelRef<'tree>> {
        self.model()
            .parent
            .and_then(|model_handle| self.tree.get(model_handle))
    }

    /// Returns true if the model as a name
    pub fn has_name(&self) -> bool {
        self.model.content.id.is_some()
    }

    /// Deduce output type from element or children.
    pub fn deduce_output_type(&self) -> ModelOutputType {
        let output_type = self.element().output_type();

        if output_type == ModelOutputType::NotDetermined {
            // Fallback: iterate over children and deduce
            for child in self.children() {
                let child_type = child.deduce_output_type();
                if child_type != ModelOutputType::NotDetermined {
                    return child_type;
                }
            }
        }

        output_type
    }

    /// Return inner group child if this model only contains a single group child.
    ///
    /// Useful for operations like `subtract() {}` or `hull() {}` to unwrap nested groups.
    pub fn into_group_child(&self) -> Option<ModelRef<'tree>> {
        let mut children = self.children();
        let first_child = children.next()?;

        // Ensure it's the ONLY child
        if children.next().is_none() && matches!(first_child.element(), Element::Group) {
            Some(first_child)
        } else {
            None
        }
    }
}

/// Iterator methods.
impl<'tree> ModelRef<'tree> {
    /// Returns an iterator over models children.
    pub fn children(&self) -> iter::Children<'tree> {
        iter::Children::new(*self)
    }

    /// Returns an iterator of models to this model and its unnamed descendants, in tree order.
    ///
    /// Includes the current model.
    pub fn unnamed_descendants(&self) -> iter::UnnamedDescendants<'tree> {
        iter::UnnamedDescendants::new(*self)
    }

    /// An iterator that descends to multiplicity nodes.
    pub fn unnamed_multiplicity_descendants(&self) -> iter::UnnamedMultiplicityDescendants<'tree> {
        iter::UnnamedMultiplicityDescendants::new(*self)
    }

    /// Parents iterator.
    pub fn parents(&self) -> iter::Parents<'tree> {
        iter::Parents::new(*self)
    }

    /// Ancestors iterator.
    pub fn ancestors(&self) -> iter::Ancestors<'tree> {
        iter::Ancestors::new(*self)
    }
}

impl<'tree> Ty for ModelRef<'tree> {
    fn ty(&self) -> Type {
        self.model().ty()
    }
}

impl<'tree> TreeDisplay for ModelRef<'tree> {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter,
        state: microcad_lang_base::TreeState,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{:indent$}{}",
            "",
            self.model(),
            indent = state.indent_spaces()
        )?;
        self.children()
            .try_for_each(|m| m.tree_print(f, state.indented()))
    }
}
