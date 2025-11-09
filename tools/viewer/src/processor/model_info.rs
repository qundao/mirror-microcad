// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Model Instance

use bevy::{
    asset::{Asset, uuid::Uuid},
    color::Color,
    ecs::component::Component,
    pbr::StandardMaterial,
    reflect::TypePath,
    transform::components::Transform,
};
use bevy_mod_outline::OutlineVolume;
use microcad_lang::{
    model::{Model, OutputType},
    render::ComputedHash,
};

use crate::{
    ToBevy,
    material::{create_2d_material, create_3d_material},
    processor::{ProcessorState, model_geometry_output_uuid},
};

#[derive(Asset, Component, Clone, Default, TypePath)]
pub struct ModelInfo {
    pub model_uuid: Uuid,
    pub geometry_output_uuid: Uuid,
    pub model_hash: u64,
    pub output_type: OutputType,
    pub transform: Transform,
    pub color: Color,
    pub outline_color: Color,
    /*     pub bounding_radius: f32,
    pub source_hash: u64,
    pub line_number: u32,

    pub is_selected: bool,
    pub is_hovered: bool,*/
}

impl ModelInfo {
    pub fn from_model(model: &Model, state: &ProcessorState) -> Self {
        let model_ = model.borrow();
        let output = model_.output();
        let transform = output.world_matrix.expect("Some matrix").to_bevy();

        let color = output
            .attributes
            .get_color()
            .cloned()
            .unwrap_or(state.theme.darker)
            .to_bevy();

        Self {
            model_uuid: super::model_uuid(model),
            model_hash: model.computed_hash(),
            geometry_output_uuid: model_geometry_output_uuid(model),
            output_type: model.render_output_type(),
            transform,
            color,
            outline_color: state.theme.bright.to_bevy(),
        }
    }

    pub fn get_default_material(&self) -> StandardMaterial {
        match self.output_type {
            OutputType::Geometry2D => create_2d_material(self.color),
            OutputType::Geometry3D => create_3d_material(self.color),
            _ => unreachable!(),
        }
    }

    pub fn get_outline(&self) -> OutlineVolume {
        match self.output_type {
            OutputType::Geometry2D => OutlineVolume {
                visible: true,
                colour: self.outline_color,
                width: 4.0,
            },
            OutputType::Geometry3D => OutlineVolume {
                visible: false,
                colour: self.outline_color,
                width: 4.0,
            },
            _ => unreachable!(),
        }
    }
}
