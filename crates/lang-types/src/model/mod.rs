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
pub use prop::{Properties, Property, PropertyType};

mod tree;

pub use tree::ModelTree;

mod operation;

pub use operation::{AffineTransform, BooleanOp};

use microcad_lang_base::{BuiltinId, HashId, Identifier, hash_id, impl_tree_types};
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

    pub hash_id: HashId,

    /// The call that created this model
    pub creator: Option<Creator>,
}

impl Model {
    pub fn primitive2d(builtin_id: BuiltinId, arguments: Arguments) -> Self {
        let hash_id = hash_id!(builtin_id, arguments);
        Self {
            name: None,
            properties: Properties::default(),
            attr: Attributes::default(),
            element: Element::BuiltinWorkpiece(element::BuiltinWorkbenchKind::Primitive2D),
            creator: Some(Creator::builtin(builtin_id, arguments)),
            hash_id,
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
        if let Some(name) = &self.name {
            write!(f, "{name}: < ")?;
        }
        write!(f, "{}", self.element)?;
        write!(f, "{}", self.attr)?;
        write!(f, "{}", self.properties)
    }
}

impl Ty for Model {
    fn ty(&self) -> crate::Type {
        Type::Model(self.output_type())
    }
}

impl_tree_types!(pub ModelTree<Model>);

/// Extension trait for [`SymbolNode`] .
pub trait NodeExt<'a> {
    fn name(&self) -> Option<&Identifier>;

    fn deduce_output_type(&self) -> ModelOutputType;

    fn into_group_child(&self) -> Option<NodeRef<'a>>;

    fn multiplicity_descendants(&self) -> iter::MultiplicityDescendants<'a>;
}

impl<'a> NodeExt<'a> for NodeRef<'a> {
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
    fn into_group_child(&self) -> Option<NodeRef<'a>> {
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
