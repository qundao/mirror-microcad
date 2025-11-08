// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer geometry output.

use bevy_mod_outline::OutlineVolume;
use microcad_core::*;
use microcad_lang::{
    model::{Model, OutputType},
    render::{ComputedHash, RenderAttributes},
};

use bevy::{
    asset::Asset,
    prelude::{Component, Mesh, StandardMaterial, Transform},
    reflect::TypePath,
};

use crate::{
    config::theme::Theme,
    processor::ProcessorState,
    to_bevy::{ToBevy, ToBevyMesh},
};

/// The output geometry from a µcad model that will be passed to Bevy.
///
/// Processing the mesh geometry will spawn bevy commands to eventually add an entity with a mesh, material and other components to a scene.
#[derive(Asset, TypePath)]
pub struct ModelOutputGeometry {
    pub mesh: Mesh,
    pub materials: ModelMaterials,

    pub aabb_mesh: Mesh,
    pub aabb_material: StandardMaterial,
    pub transform: Transform,

    pub info: ModelInfo,
}

#[derive(Component)]
pub struct ModelMaterials {
    /// Default material.
    pub default: StandardMaterial,
    /// Material when a model is supposed to be drawn in the background.
    pub transparent: StandardMaterial,
    /// The outline.
    pub outline: OutlineVolume,
}

impl ModelMaterials {
    /// Create new model materials from render attributes.
    pub fn new(output_type: &OutputType, attributes: &RenderAttributes, theme: &Theme) -> Self {
        let color = attributes.get_color().cloned().unwrap_or(theme.brighter);
        let transparent_color = color.make_transparent(color.a * 0.3);
        use crate::material::{create_2d_material, create_3d_material};

        match output_type {
            OutputType::Geometry2D => Self {
                default: create_2d_material(&color),
                transparent: create_2d_material(&transparent_color),
                outline: OutlineVolume {
                    visible: true,
                    colour: theme.bright.to_bevy(),
                    width: 4.0,
                },
            },
            OutputType::Geometry3D => Self {
                default: create_3d_material(&color),
                transparent: create_3d_material(&transparent_color),
                outline: OutlineVolume {
                    visible: true,
                    colour: theme.bright.to_bevy(),
                    width: 4.0,
                },
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Component)]
pub struct ModelInfo {
    pub model_hash: u64, // It might be useful to have the model hash as reference to a specific model node.
    pub bounding_radius: f32,
    pub output_type: OutputType,
}

impl ModelOutputGeometry {
    /// Create [`OutputGeometry`] from µcad model.
    pub fn from_model(model: &Model, state: &ProcessorState) -> Option<Self> {
        use microcad_lang::model::Element::*;
        use microcad_lang::render::GeometryOutput;

        let model_ = model.borrow();
        // We only consider output geometries of workpieces and ignore the rest.
        match model_.element() {
            InputPlaceholder | Multiplicity | Group => {
                return None;
            }
            Workpiece(_) | BuiltinWorkpiece(_) => {}
        }

        let output = model_.output();
        let output_type = output.output_type;
        let transform = output.world_matrix.expect("Some matrix").to_bevy();
        let materials = ModelMaterials::new(&output_type, &output.attributes, &state.theme);
        let aabb_material = crate::material::create_2d_material(&Color::rgb(1.0, 1.0, 1.0));

        match &output.geometry {
            Some(GeometryOutput::Geometry2D(geometry)) => Some(Self {
                mesh: geometry.inner.to_bevy_mesh_default(),
                materials,
                aabb_mesh: geometry.bounds.to_bevy_mesh_default(),
                aabb_material,
                transform,
                info: ModelInfo {
                    model_hash: model.computed_hash(),
                    bounding_radius: geometry.bounds.radius() as f32,
                    output_type,
                },
            }),
            Some(GeometryOutput::Geometry3D(geometry)) => Some(Self {
                mesh: geometry.inner.to_bevy_mesh(30.0),
                materials,
                aabb_mesh: geometry.bounds.to_bevy_mesh_default(),
                aabb_material,
                transform,
                info: ModelInfo {
                    model_hash: model.computed_hash(),
                    bounding_radius: geometry.bounds.radius() as f32,
                    output_type,
                },
            }),
            None => None,
        }
    }
}
