// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render attributes.

use derive_more::Deref;
use microcad_core::Color;

/// An attribute that can be used by any renderer.
///
/// *Note: Render color is the only supported attribute for now.*
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum RenderAttribute {
    /// Color attribute.
    Color(Color),
}

impl RenderAttribute {
    fn same_variant(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
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
