// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Grid material.

use super::bevy_types::*;

/// A colored zoom-adaptive grid and fade out radius.
///
/// This struct defines the data that will be passed to your shader.
#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
pub struct Grid {
    #[uniform(0)]
    pub radius: f32,

    #[uniform(1)]
    pub zoom_level: f32,

    #[uniform(2)]
    pub view_angle: Vec3,

    #[uniform(3)]
    pub grid_color: Vec3,

    #[uniform(4)]
    pub x_axis_color: Vec3,

    #[uniform(5)]
    pub y_axis_color: Vec3,
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
            grid_color: Vec3::new(0.7, 0.7, 0.7),
            x_axis_color: Vec3::new(1.0, 0.0, 0.0),
            y_axis_color: Vec3::new(0.0, 1.0, 0.0),
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
        AlphaMode::Blend
    }
}
