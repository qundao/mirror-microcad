// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render attributes.

use microcad_core::Color;

/// An attribute that can be used by any renderer.
///
/// *Note: Render color is the only supported attribute for now.*
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct RenderAttributes {
    /// Color attribute.
    pub color: Color,
}
