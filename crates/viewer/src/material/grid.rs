// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Grid material.

use bevy::render::render_resource::RenderPipelineDescriptor;

use super::bevy_types::*;

/// A colored zoom-adaptive grid and fade out radius.
///
/// This struct defines the data that will be passed to your shader.
#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
pub struct Grid {
    /// Grid fade out radius.
    #[uniform(0)]
    pub radius: f32,

    /// Zoom level (defines the tile size of the grid).
    #[uniform(1)]
    pub zoom_level: f32,

    /// Camera view angle.
    #[uniform(2)]
    pub view_angle: Vec3,

    /// RGB color of the grid.
    #[uniform(3)]
    pub grid_color: Vec3,

    /// RGB Color for the x axis.
    #[uniform(4)]
    pub x_axis_color: Vec3,

    /// RGB Color for the y axis.
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

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None; // ✅ Disable backface culling
        Ok(())
    }
}
