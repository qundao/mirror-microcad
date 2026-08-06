// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

use derive_more::{Deref, Display};

pub mod attribute;
pub mod creator;
pub mod element;
pub mod iter;
pub mod operation;
pub mod ops;
pub mod output_type;
pub mod workpiece;

use microcad_lang_base::{Identifier, TreeDisplay, TreeState, element::Visibility};
use serde::{Deserialize, Serialize};

pub use attribute::Attributes;

pub use element::{Element, ElementKind};

pub use creator::Creator;
pub use output_type::OutputType;

use crate::{Ty, Type, Value};

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
            inner: model.inner,
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
        if let Some(parent_handle) = parent {
            if let Some(parent_node) = self.get_mut(parent_handle) {
                parent_node.children.insert(first_inserted_handle);
            }
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

impl From<Model> for ModelTree {
    fn from(root: Model) -> Self {
        Self { nodes: vec![root] }
    }
}

impl TreeDisplay for ModelTree {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        match self.root() {
            Some(root) => root.tree_print(f, depth),
            None => write!(f, "<EMPTY TREE>"),
        }
    }
}

impl std::fmt::Display for ModelTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, TreeState::new_display())
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

    /// Deduce output type from element or children.
    pub fn deduce_output_type(&self) -> OutputType {
        let output_type = self.element().output_type();

        if output_type == OutputType::NotDetermined {
            // Fallback: iterate over children and deduce
            for child in self.children() {
                let child_type = child.deduce_output_type();
                if child_type != OutputType::NotDetermined {
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
        if children.next().is_none() && matches!(first_child.element().kind(), ElementKind::Group) {
            Some(first_child)
        } else {
            None
        }
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

#[derive(Debug, Display, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[display("{inner}")]
pub struct Model {
    /// Parent of the model
    pub parent: Option<ModelHandle>,

    /// The actual model content
    pub inner: ModelInner,

    /// Children of this model
    pub children: Models,
}

impl Model {
    pub fn with_name(mut self, id: Identifier) -> Self {
        self.inner.id = Some(id);
        self
    }

    pub fn with_visibility(mut self, vis: Visibility) -> Self {
        self.inner.visibility = vis;
        self
    }

    pub fn with_attr(mut self, attr: Attributes) -> Self {
        self.inner.attr = attr;
        self
    }

    pub fn output_type(&self) -> OutputType {
        self.inner.element.output_type()
    }
}

impl Ty for Model {
    fn ty(&self) -> crate::Type {
        Type::Model(self.output_type())
    }
}

impl From<Value> for Model {
    fn from(value: Value) -> Self {
        Model {
            parent: None,
            inner: ModelInner {
                id: None,
                visibility: Default::default(),
                attr: Default::default(),
                element: Element::from(value),
            },
            children: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
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

impl std::fmt::Display for ModelInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.attr)?;
        if Visibility::Public == self.visibility {
            write!(f, "prop ")?;
        }
        if let Some(id) = &self.id {
            write!(f, "{id} = ")?;
        }

        write!(f, "{}", self.element)
    }
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
