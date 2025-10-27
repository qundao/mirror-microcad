// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer bevy systems

use bevy::render::mesh::{Mesh, Mesh3d};
use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        event::{EventReader, EventWriter},
        system::{Commands, Res, ResMut},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
};
use bevy_mod_outline::{OutlineMode, OutlineVolume};
use microcad_core::RenderResolution;

use crate::{
    processor::{ProcessorRequest, ProcessorResponse},
    scene::{Scene, SceneRadiusChangeEvent},
    state::State,
};

/// Start up the processor.
pub fn startup_processor(
    mut event_writer: EventWriter<crate::processor::ProcessorRequest>,
    state: ResMut<crate::state::State>,
) {
    state
        .processor
        .send_request(ProcessorRequest::Initialize {
            search_paths: state.settings.search_paths.clone(),
            path: state.input.clone(),
            resolution: RenderResolution::default(),
        })
        .expect("No error");

    event_writer.write(ProcessorRequest::Render);
}

/// This system handles responses coming from the processor and fills the Bevy command pipeline.
pub fn handle_processor_responses(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut state: ResMut<State>,
    mut event_writer: EventWriter<SceneRadiusChangeEvent>,
) {
    let mut entities = Vec::new();
    let mut new_entities = false;
    let mut new_scene_radius = Scene::MINIMUM_RADIUS;

    for response in state.processor.response_receiver.try_iter() {
        match response {
            ProcessorResponse::OutputGeometry(mesh_geometry_outputs) => {
                // Despawn all entities to remove them from the scene
                for entity in &state.scene.model_entities {
                    commands.entity(*entity).despawn();
                }
                new_entities = true;

                for mesh_geometry_output in mesh_geometry_outputs {
                    new_scene_radius = new_scene_radius.max(mesh_geometry_output.bounding_radius);

                    let entity = commands.spawn((
                        Mesh3d(meshes.add(mesh_geometry_output.mesh)),
                        MeshMaterial3d(materials.add(mesh_geometry_output.material)),
                        OutlineVolume {
                            visible: matches!(
                                mesh_geometry_output.output_type,
                                microcad_lang::model::OutputType::Geometry2D
                            ),
                            colour: Color::srgba(0.1, 0.1, 0.1, 1.0),
                            width: 4.0,
                        },
                        OutlineMode::FloodFlat,
                    ));
                    /*entity.observe(
                        move |trigger: Trigger<Pointer<Click>>,
                              mut materials: ResMut<Assets<StandardMaterial>>,
                              mut query: Query<(
                            &mut MeshMaterial3d<StandardMaterial>,
                            &mut OutlineVolume,
                        )>| {
                            if let Ok((material, mut component, mut outline)) =
                                query.get_mut(trigger.target())
                                && let Some(material) = materials.get_mut(material.id())
                            {
                                component.selected = !component.selected;
                                outline.visible = component.selected;
                                match component.selected {
                                    true => material.base_color = Color::srgba(1.0, 1.0, 2.0, 1.0),
                                    false => material.base_color = Color::srgba(1.0, 1.0, 1.0, 1.0),
                                }
                            }
                        },
                    );*/

                    entities.push(entity.id());
                }
            }
        }

        if new_entities {
            break;
        }
    }

    if new_entities {
        state.scene.model_entities = entities;
        state.scene.radius = new_scene_radius;
        event_writer.write(SceneRadiusChangeEvent {
            new_radius: new_scene_radius,
        });
    }
}

pub fn handle_processor_request(
    mut event_reader: EventReader<ProcessorRequest>,
    state: Res<State>,
) {
    for event in event_reader.read() {
        if let Err(error) = state.processor.send_request(event.clone()) {
            log::error!("Render error: {error}");
        }
    }
}
