// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

use derive_more::{Deref, Display};

pub mod attribute;
pub mod creator;
pub mod element;
pub mod iter;
pub mod ops;
pub mod output_type;
pub mod workpiece;

mod model_ref;
pub use model_ref::ModelRef;

mod operation;
pub use operation::{AffineTransform, BooleanOp};

mod tree;
pub use tree::{ModelHandle, ModelTree};

use microcad_lang_base::{Identifier, element::Visibility};
use serde::{Deserialize, Serialize};

pub use attribute::Attributes;

pub use element::{Element, ElementKind};

pub use creator::Creator;
pub use output_type::ModelOutputType;

use crate::{Ty, Type, Value};

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

    pub fn output_type(&self) -> ModelOutputType {
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
