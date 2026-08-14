// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

pub mod attribute;
pub mod creator;
pub mod element;
pub mod iter;
pub mod output_type;
pub mod workpiece;

mod prop;
pub use prop::{Properties, Property};

mod tree;

pub use tree::ModelTree;

mod operation;

pub use operation::{AffineTransform, BooleanOp};

use microcad_lang_base::{BuiltinId, Identifier};
use serde::{Deserialize, Serialize};

pub use attribute::Attributes;
pub use creator::Creator;
pub use element::Element;
pub use output_type::ModelOutputType;

use crate::{Arguments, Ty, Type};

#[derive(Debug, Default, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Model {
    /// An optional name
    pub name: Option<Identifier>,

    /// Model attributes `#`
    pub attr: Attributes,

    /// Model properties `-`
    pub properties: Properties,

    /// Model element
    pub element: Element,

    /// The call that created this model
    pub creator: Option<Creator>,
}

impl Model {
    pub fn primitive2d(builtin_id: BuiltinId, arguments: Arguments) -> Model {
        Model {
            name: None,
            properties: Properties::default(),
            attr: Attributes::default(),
            element: Element::BuiltinWorkpiece(element::BuiltinWorkbenchKind::Primitive2D),
            creator: Some(Creator::builtin(builtin_id, arguments)),
        }
    }

    pub fn with_name(mut self, name: Identifier) -> Self {
        self.name = Some(name);
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

pub type ModelArena = microcad_lang_base::tree::Arena<Model>;
pub type ModelNode = microcad_lang_base::tree::Node<Model>;
pub type ModelNodeRef<'a> = microcad_lang_base::tree::NodeRef<'a, Model>;
pub type ModelNodeMut<'a> = microcad_lang_base::tree::NodeMut<'a, Model>;
pub type ModelNodeId = microcad_lang_base::tree::NodeId;

/// Extension trait for [`SymbolNode`] .
pub trait ModelNodeExt<'a> {
    fn name(&self) -> Option<&Identifier>;

    fn deduce_output_type(&self) -> ModelOutputType;

    fn into_group_child(&self) -> Option<ModelNodeRef<'a>>;

    fn multiplicity_descendants(&self) -> iter::MultiplicityDescendants<'a>;
}

impl<'a> ModelNodeExt<'a> for ModelNodeRef<'a> {
    fn name(&self) -> Option<&Identifier> {
        self.name.as_ref()
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
