// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer bevy systems

use bevy::render::mesh::{Mesh, Mesh3d};
use bevy::{
    asset::Assets,
    ecs::{
        event::{EventReader, EventWriter},
        system::{Commands, Res, ResMut},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
};
use bevy_mod_outline::OutlineMode;

use crate::stdin::StdinMessageReceiver;
use crate::*;
use crate::{
    processor::{ProcessorRequest, ProcessorResponse},
    scene::SceneRadiusChangeEvent,
};

/// Whether a kind of watch event is relevant for compilation.
fn is_relevant_event_kind(kind: &notify::EventKind) -> bool {
    match kind {
        notify::EventKind::Any => false,
        notify::EventKind::Access(_) => false,
        notify::EventKind::Create(_) => true,
        notify::EventKind::Modify(kind) => match kind {
            notify::event::ModifyKind::Any => true,
            notify::event::ModifyKind::Data(_) => true,
            notify::event::ModifyKind::Metadata(_) => true,
            notify::event::ModifyKind::Name(_) => true,
            notify::event::ModifyKind::Other => false,
        },
        notify::EventKind::Remove(_) => true,
        notify::EventKind::Other => false,
    }
}

/// Start up the processor.
///
/// Sends an initialize request to the processor and handles input.
pub fn initialize_processor(mut state: ResMut<crate::state::State>) {
    state
        .processor
        .send_request(ProcessorRequest::Initialize {
            config: state.config.clone(),
        })
        .expect("No error");

    use crate::plugin::MicrocadPluginInput::*;
    let flag_clone = state.last_modified.clone();
    let reload_delay = state.config.reload_delay;

    let mut requests = Vec::new();

    match &mut state.input {
        Some(File {
            path,
            symbol: _,
            line,
        }) => {
            let path = path.clone();
            requests.push(ProcessorRequest::ParseFile(path.clone()));
            requests.push(ProcessorRequest::SetLineNumber(*line));

            // Run file watcher thread.
            std::thread::spawn(move || -> ! {
                use notify::{RecursiveMode, Watcher};

                let (tx, rx) = std::sync::mpsc::channel();
                let mut watcher = notify::recommended_watcher(tx).unwrap();
                watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

                log::info!("Watching external file: {}", path.display());

                loop {
                    if let Ok(Ok(event)) = rx.recv_timeout(reload_delay)
                        && is_relevant_event_kind(&event.kind)
                        && let Ok(meta) = std::fs::metadata(&path)
                        && let Ok(modified) = meta.modified()
                    {
                        log::info!("Modified");
                        *flag_clone.lock().unwrap() = Some(modified);
                        watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();
                    }
                }
            });
        }
        Some(crate::plugin::MicrocadPluginInput::Stdin(stdin)) => {
            log::info!("Run viewer in stdin remote controlled mode.");
            *stdin = Some(StdinMessageReceiver::run());
        }
        _ => { /* Do nothing */ }
    }

    requests
        .into_iter()
        .for_each(|request| state.processor.send_request(request).expect("No error"));
}

pub fn handle_external_reload(
    mut event_writer: EventWriter<ProcessorRequest>,
    state: ResMut<crate::state::State>,
) {
    use crate::plugin::MicrocadPluginInput::*;

    match &state.input {
        Some(File { path, .. }) => {
            let mut last_modified_lock = state.last_modified.lock().unwrap();
            if let Some(last_modified) = *last_modified_lock
                && let Ok(elapsed) = last_modified.elapsed()
                && elapsed > state.config.reload_delay
            {
                event_writer.write(ProcessorRequest::ParseFile(path.to_path_buf()));
                log::info!("Changed file");

                // Reset so we don’t reload again
                *last_modified_lock = None;
            }
        }
        _ => { /* Do nothing */ }
    }
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
    let mut new_scene_radius = scene::Scene::MINIMUM_RADIUS;

    for response in state.processor.response_receiver.try_iter() {
        match response {
            ProcessorResponse::NewModelGeometry(model_geometry_output) => {
                new_entities = true;
                new_scene_radius = new_scene_radius.max(model_geometry_output.info.bounding_radius);

                // Spawn object entity.
                entities.push(
                    commands
                        .spawn((
                            Mesh3d(meshes.add(model_geometry_output.mesh)),
                            MeshMaterial3d(materials.add(model_geometry_output.materials.default)),
                            model_geometry_output.transform,
                            model_geometry_output.materials.outline,
                            OutlineMode::FloodFlat,
                        ))
                        .id(),
                );
            }
            ProcessorResponse::OutputGeometryId(id) => {
                log::info!("This output geometries has alredy been rendered, just respawn {id}");
            }
        }

        if state.processor.response_receiver.is_empty() {
            break;
        }
    }

    if new_entities {
        // Despawn all entities to remove them from the scene
        for entity in &state.scene.model_entities {
            commands.entity(*entity).despawn();
        }
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
