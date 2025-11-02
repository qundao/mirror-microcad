// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render attributes.

use derive_more::Deref;
use microcad_core::Color;

use crate::model::{Attributes, Model};

/// An attribute that can be used by any renderer.
///
/// *Note: Render color is the only supported attribute for now.*
#[derive(Clone, Debug)]
pub enum RenderAttribute {
    /// Color attribute.
    Color(Color),
}

impl RenderAttribute {
    fn same_variant(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (RenderAttribute::Color(_), RenderAttribute::Color(_))
        )
    }
}

/// A list of render attributes.
///
/// Each enum variant of [`RenderAttribute`] can only be present at most once in the attribute list.
#[derive(Clone, Debug, Default, Deref)]
pub struct RenderAttributes(Vec<RenderAttribute>);

impl RenderAttributes {
    /// Insert a render attribute and overwrite old attribute if present.
    pub fn insert(&mut self, attr: RenderAttribute) {
        // remove existing variant of the same type
        self.0.retain(|a| !a.same_variant(&attr));
        self.0.push(attr);
    }

    /// Get color from color attribute, if any.
    pub fn get_color(&self) -> Option<&Color> {
        self.0
            .iter()
            .map(|attr| match attr {
                RenderAttribute::Color(color) => color,
            })
            .next()
    }
}

impl From<&Attributes> for RenderAttributes {
    fn from(attributes: &Attributes) -> Self {
        use crate::model::Attribute;
        let mut render_attributes = RenderAttributes::default();
        attributes.iter().for_each(|attr| {
            if let Attribute::Color(color) = attr {
                render_attributes.insert(RenderAttribute::Color(*color))
            }
        });

        render_attributes
    }
}

impl From<Model> for RenderAttributes {
    fn from(model: Model) -> Self {
        let model_ = model.borrow();
        let mut render_attributes: RenderAttributes = model_.attributes().into();

        if render_attributes.is_empty() {
            if let Some(child) = model_.children.single_model() {
                render_attributes = child.borrow().attributes().into();
            }
        }

        render_attributes
    }
}
