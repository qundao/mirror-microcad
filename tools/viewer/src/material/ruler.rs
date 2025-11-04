// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ruler material.

use super::bevy_types::*;

#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
// This struct defines the data that will be passed to your shader
pub struct Ruler {
    #[uniform(0)]
    zoom_level: f32,

    alpha_mode: AlphaMode,
}

impl Default for Ruler {
    fn default() -> Self {
        Self {
            zoom_level: 1.0,
            alpha_mode: AlphaMode::Blend,
        }
    }
}

impl Ruler {
    fn shader_ref() -> ShaderRef {
        super::shader_ref_from_str("ruler.wgsl")
    }
}

impl Material for Ruler {
    fn fragment_shader() -> ShaderRef {
        Self::shader_ref()
    }

    fn vertex_shader() -> ShaderRef {
        Self::shader_ref()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
