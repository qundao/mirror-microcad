// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Grid material.

use super::bevy_types::*;

#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
// This struct defines the data that will be passed to your shader
pub struct Grid {
    #[uniform(0)]
    pub radius: f32,

    #[uniform(1)]
    pub zoom_level: f32,

    #[uniform(2)]
    pub view_angle: Vec3,

    alpha_mode: AlphaMode,
}

impl Grid {
    fn shader_ref() -> ShaderRef {
        super::shader_ref_from_str("grid.wgsl")
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            radius: 1.0,
            zoom_level: 1.0,
            view_angle: Vec3::new(0.0, 0.0, 1.0),
            alpha_mode: AlphaMode::Blend,
        }
    }
}

impl Material for Grid {
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
