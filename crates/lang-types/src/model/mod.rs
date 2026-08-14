// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

pub mod attribute;
pub mod creator;
pub mod element;
pub mod iter;
pub mod ops;
pub mod output_type;
pub mod workpiece;

mod operation;
use std::collections::BTreeMap;

pub use operation::{AffineTransform, BooleanOp};

use microcad_lang_base::{BuiltinId, Identifier, element::Visibility};
use serde::{Deserialize, Serialize};

pub use attribute::Attributes;
pub use creator::Creator;
pub use element::Element;
pub use output_type::ModelOutputType;

use crate::{Arguments, Ty, Type, Value};

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Model {
    /// An optional name
    pub name: Option<Identifier>,

    pub properties: BTreeMap<Identifier, Value>,

    /// Visibility
    pub visibility: Visibility,

    /// Model attributes
    pub attr: Attributes,

    /// Model element
    pub element: Element,

    /// The call that created this model
    pub creator: Option<Creator>,
}

impl Model {
    pub fn primitive2d(builtin_id: BuiltinId, arguments: Arguments) -> Model {
        Model {
            name: None,
            properties: BTreeMap::default(),
            visibility: Visibility::Private,
            attr: Attributes::default(),
            element: Element::BuiltinWorkpiece(element::BuiltinWorkbenchKind::Primitive2D),
            creator: Some(Creator::builtin(builtin_id, arguments)),
        }
    }

    pub fn with_name(mut self, name: Identifier) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_visibility(mut self, vis: Visibility) -> Self {
        self.visibility = vis;
        self
    }

    pub fn with_attr(mut self, attr: Attributes) -> Self {
        self.attr = attr;
        self
    }

    pub fn output_type(&self) -> ModelOutputType {
        self.element.output_type()
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.attr)?;
        if Visibility::Public == self.visibility {
            write!(f, "prop ")?;
        }
        if let Some(name) = &self.name {
            write!(f, "{name} = ")?;
        }

        write!(f, "{}", self.element)
    }
}

impl Ty for Model {
    fn ty(&self) -> crate::Type {
        Type::Model(self.output_type())
    }
}

use microcad_lang_base::tree;

pub type ModelArena = tree::Arena<Model>;
pub type ModelNode = tree::Node<Model>;
pub type ModelNodeRef<'a> = tree::NodeRef<'a, Model>;
pub type ModelNodeMut<'a> = tree::NodeMut<'a, Model>;
pub type ModelNodeId = tree::NodeId;

/// Extension trait for [`SymbolNode`] .
pub trait ModelNodeExt<'a> {
    fn name(&self) -> Option<&Identifier>;

    fn is_public(&self) -> bool;

    fn deduce_output_type(&self) -> ModelOutputType;

    fn into_group_child(&self) -> Option<ModelNodeRef<'a>>;

    fn multiplicity_descendants(&self) -> iter::MultiplicityDescendants<'a>;
}

impl<'a> ModelNodeExt<'a> for ModelNodeRef<'a> {
    fn name(&self) -> Option<&Identifier> {
        self.name.as_ref()
    }

    fn is_public(&self) -> bool {
        self.visibility.is_public()
    }

    /// Deduce output type from element or children.
    fn deduce_output_type(&self) -> ModelOutputType {
        let output_type = self.element.output_type();

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
    fn into_group_child(&self) -> Option<ModelNodeRef<'a>> {
        let mut children = self.children();
        let first_child = children.next()?;

        // Ensure it's the ONLY child
        if children.next().is_none() && matches!(first_child.element, Element::Group) {
            Some(first_child)
        } else {
            None
        }
    }

    /// An iterator that descends to multiplicity nodes.
    fn multiplicity_descendants(&self) -> iter::MultiplicityDescendants<'a> {
        iter::MultiplicityDescendants::new(*self)
    }
}

/// A model tree with a root node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelTree {
    root: ModelNodeId,
    pub arena: ModelArena,
}

impl ModelTree {
    pub fn new(root: Model) -> Self {
        let mut arena = ModelArena::new();

        Self {
            root: arena.new_node(root),
            arena,
        }
    }

    pub fn root<'a>(&'a self) -> ModelNodeRef<'a> {
        ModelNodeRef::new(self.root, &self.arena)
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

/*
impl Model {
    /// Short cut to generate boolean operator as binary operation with two models.
    pub fn boolean_op(self, op: BooleanOp, other: Model) -> ModelTree {
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
}*/
