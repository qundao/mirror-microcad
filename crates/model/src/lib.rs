// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

use derive_more::Deref;

pub mod attribute;
pub mod creator;
pub mod element;
pub mod iter;
pub mod operation;
pub mod ops;
pub mod output_type;
pub mod workpiece;

use microcad_lang_base::{Identifier, element::Visibility};
use serde::{Deserialize, Serialize};

use crate::{attribute::Attributes, element::Element};

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

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn insert(&mut self, parent: Option<ModelHandle>, tree: impl Into<ModelTree>) {
        let tree = tree.into();

        let base_index = self.nodes.len();
        let handle = ModelHandle(base_index);

        let nodes = tree.nodes.into_iter().map(|model| Model {
            inner: model.inner,
            parent: model
                .parent
                .map(|ModelHandle(index)| ModelHandle(index + base_index))
                .or(parent),
            children: Models::from_iter(
                model
                    .children
                    .items
                    .into_iter()
                    .map(|ModelHandle(index)| ModelHandle(index + base_index)),
            ),
        });
        self.nodes.extend(nodes.into_iter());

        if let Some(parent) = parent {
            if let Some(parent_node) = self.get_mut(parent) {
                parent_node.children.insert(handle);
            }
        }
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

impl From<Model> for ModelTree {
    fn from(root: Model) -> Self {
        Self { nodes: vec![root] }
    }
}

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
        &self.model.inner.element
    }

    /// Return the parent of this model.
    pub fn parent(&self) -> Option<ModelRef<'tree>> {
        self.model()
            .parent
            .and_then(|model_handle| self.tree.get(model_handle))
    }

    /// Returns true if the model as a name
    pub fn has_name(&self) -> bool {
        self.model.inner.id.is_some()
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Models {
    pub items: Vec<ModelHandle>,
}
impl Models {
    fn insert(&mut self, handle: ModelHandle) {
        self.items.push(handle);
    }
}

impl FromIterator<ModelHandle> for Models {
    fn from_iter<T: IntoIterator<Item = ModelHandle>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct Model {
    /// Parent of the model
    pub parent: Option<ModelHandle>,

    /// The actual model content
    pub inner: ModelInner,

    /// Children of this model
    pub children: Models,
}

#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct ModelInner {
    /// An optional id
    pub id: Option<Identifier>,

    /// Visibility
    pub visibility: Visibility,

    /// Model attributes
    pub attr: Attributes,

    /// Model elements
    pub element: Element,
}

/*
impl Model {
    /// Short cut to generate boolean operator as binary operation with two models.
    pub fn boolean_op(self, op: BooleanOp, other: Model) -> ModelTree {
        assert!(self != other, "lhs and rhs must be distinct.");
        ModelTree::from(vec![self.clone(), other]).boolean_op(op)
    }

    /// Multiply a model n times.
    pub fn multiply(&self, n: Integer) -> Vec<Model> {
        (0..n).map(|_| self.make_deep_copy()).collect()
    }

    /// Replace each input placeholder with copies of `input_model`.
    pub fn replace_input_placeholders(&self, input_model: &Model) -> Self {
        self.descendants().for_each(|model| {
            let mut model_ = model.borrow_mut();
            if model_.id.is_none() && matches!(model_.element.value, Element::InputPlaceholder) {
                let input_model_ = input_model.borrow_mut();
                *model_ = input_model_.clone_content();
                model_.parent = Some(self.clone());
                model_.children = input_model_.children.clone();
            }
        });
        self.clone()
    }

    /// Deduce output type from children and set it and return it.
    pub fn deduce_output_type(&self) -> OutputType {
        let self_ = self.borrow();
        let mut output_type = self_.element.output_type();
        if output_type == OutputType::NotDetermined {
            let children = &self_.children;
            output_type = children.deduce_output_type();
        }

        output_type
    }

    /// Return inner group if this model only contains a group as single child.
    ///
    /// This function is used when we evaluate operations like `subtract() {}` or `hull() {}`.
    /// When evaluating these operations, we want to iterate over the group's children.
    pub fn into_group(&self) -> Option<Model> {
        self.borrow()
            .children
            .single_model()
            .filter(|model| matches!(model.borrow().element.value, Element::Group))
    }

    /// Set the id of a model. This happens if the model was created by an assignment.
    ///
    /// For example, the assignment statement `a = Circle(4mm)` will result in a model with id `a`.
    pub fn set_id(&self, id: Identifier) {
        self.borrow_mut().id = Some(id);
    }
}*/

/// Iterator methods.
impl<'tree> ModelRef<'tree> {
    /// Returns an iterator over models children.
    pub fn children(&self) -> iter::Children<'tree> {
        iter::Children::new(self.clone())
    }

    /// Returns an iterator of models to this model and its unnamed descendants, in tree order.
    ///
    /// Includes the current model.
    pub fn unnamed_descendants(&self) -> iter::UnnamedDescendants<'tree> {
        iter::UnnamedDescendants::new(self.clone())
    }

    /// An iterator that descends to multiplicity nodes.
    pub fn unnamed_multiplicity_descendants(&self) -> iter::UnnamedMultiplicityDescendants<'tree> {
        iter::UnnamedMultiplicityDescendants::new(self.clone())
    }

    /// Returns an iterator of models that belong to the same source file as this one
    /*pub fn source_file_descendants(&self) -> SourceFileDescendants<'tree> {
        SourceFileDescendants::new(self.clone())
    }*/

    /// Parents iterator.
    pub fn parents(&self) -> iter::Parents<'tree> {
        iter::Parents::new(self.clone())
    }

    /// Ancestors iterator.
    pub fn ancestors(&self) -> iter::Ancestors<'tree> {
        iter::Ancestors::new(self.clone())
    }
}
