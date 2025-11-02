// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer geometry output.

use microcad_core::*;
use microcad_lang::{
    model::{Model, OutputType},
    render::ComputedHash,
};

use crate::processor::mesh_instance::MeshInstance;

/// The output geometry from a µcad model that will be passed to Bevy.
///
/// Processing the mesh geometry will spawn bevy commands to eventually add an entity with a mesh, material and other components to a scene.
pub struct OutputGeometry {
    pub model_hash: u64, // It might be useful to have the model hash as reference to a specific model node.
    pub output_type: OutputType,
    pub object: MeshInstance,
    pub aabb: MeshInstance,

    pub bounding_radius: f32,
}

impl OutputGeometry {
    /// Create [`OutputGeometry`] from µcad model.
    pub fn from_model(model: &Model) -> Option<Self> {
        let model_ = model.borrow();
        let output = model_.output();
        let mat = output.world_matrix.expect("Some matrix");
        let output_type = output.output_type;
        use microcad_lang::render::GeometryOutput;

        let color = output.attributes.get_color().cloned().unwrap_or_default();

        match &output.geometry {
            Some(GeometryOutput::Geometry2D(geometry)) => Some(Self {
                model_hash: model.computed_hash(),
                object: MeshInstance::new_2d(&geometry.inner, color, mat),
                aabb: MeshInstance::new_bounds_2d(
                    geometry.bounds.clone(),
                    Color::rgb(1.0, 1.0, 1.0),
                    mat,
                ),
                output_type,
                bounding_radius: geometry.bounds.radius() as f32,
            }),
            Some(GeometryOutput::Geometry3D(geometry)) => Some(Self {
                model_hash: model.computed_hash(),
                object: MeshInstance::new_3d(&geometry.inner, color, mat),
                aabb: MeshInstance::new_bounds_3d(
                    geometry.bounds.clone(),
                    Color::rgb(1.0, 1.0, 1.0),
                    mat,
                ),
                output_type,
                bounding_radius: geometry.bounds.radius() as f32,
            }),
            None => None,
        }
    }
}
