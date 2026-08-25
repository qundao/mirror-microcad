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

pub use tree::{
    Arena, BuildModelTreeError, ModelTree, ModelTreeBuilder, ModelTreeBuilderMut, Node, NodeExt,
    NodeId, NodeMut, NodeRef,
};

mod operation;

pub use operation::{AffineTransform, BooleanOp};

use microcad_lang_base::{HashId, Identifier};
use serde::{Deserialize, Serialize};

pub use attribute::{Attribute, AttributeAccess, Attributes};

pub use creator::Creator;
pub use element::Element;
pub use output_type::ModelOutputType;

use crate::{Arguments, Ty, Type, Value};

#[derive(Debug, Default, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Model {
    /// An optional name
    pub name: Option<Identifier>,

    /// Model attributes `#`
    pub attr: Attributes,

    /// Model properties
    pub properties: Properties,

    /// Model element
    pub element: Element,

    pub hash_id: HashId,

    /// The call that created this model
    pub creator: Option<Creator>,
}

/// Builder functions
impl Model {
    pub fn new(element: impl Into<Element>) -> Self {
        Self {
            element: element.into(),
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: impl Into<Identifier>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_attr(mut self, attr: impl Into<Attributes>) -> Self {
        self.attr = attr.into();
        self
    }

    pub fn with_op_properties(mut self, args: Arguments) -> Self {
        let args = args.with_field_removed("self");
        self.properties = Properties::hidden_inputs(args);
        self
    }

    pub fn with_properties(mut self, properties: impl Into<Properties>) -> Self {
        self.properties = properties.into();
        self
    }
}

/// Accessor functions
impl Model {
    pub fn output_type(&self) -> ModelOutputType {
        self.element.output_type()
    }

    pub fn get_property(&self, name: impl AsRef<str>) -> Option<&Property> {
        self.properties.get_property(name.as_ref())
    }
}

impl AttributeAccess for Model {
    fn get_attribute(&self, name: impl AsRef<str>) -> Option<Value> {
        self.attr.get_attribute(name)
    }
}

impl From<Element> for Model {
    fn from(element: Element) -> Self {
        Self::new(element)
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
