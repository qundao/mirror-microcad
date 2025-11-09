// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer bevy systems

use bevy::render::mesh::{Mesh, Mesh3d};
use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res, ResMut},
    pbr::StandardMaterial,
    prelude::*,
};
use bevy_mod_outline::{OutlineMode, OutlineVolume};

use crate::processor::model_info::ModelInfo;
use crate::processor::{ProcessorRequest, ProcessorResponse};
use crate::stdin::StdinMessageReceiver;
use crate::*;

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

    use crate::plugin::MicrocadPluginInput;
    let reload_delay = state.config.reload_delay;

    let mut requests = Vec::new();

    match &mut state.input {
        Some(MicrocadPluginInput::File {
            path,
            symbol: _,
            line,
            last_modified,
        }) => {
            let flag_clone = last_modified.clone();
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
        Some(MicrocadPluginInput::Stdin(stdin)) => {
            log::info!("Run viewer in stdin remote controlled mode.");
            *stdin = Some(StdinMessageReceiver::run());
        }
        _ => { /* Do nothing */ }
    }

    requests
        .into_iter()
        .for_each(|request| state.processor.send_request(request).expect("No error"));
}

pub fn handle_external_reload(state: ResMut<crate::state::State>) {
    use crate::plugin::MicrocadPluginInput::*;

    match &state.input {
        Some(File {
            path,
            last_modified,
            ..
        }) => {
            let mut last_modified_lock = last_modified.lock().unwrap();
            if let Some(last_modified) = *last_modified_lock
                && let Ok(elapsed) = last_modified.elapsed()
                && elapsed > state.config.reload_delay
            {
                state
                    .processor
                    .send_request(ProcessorRequest::ParseFile(path.to_path_buf()))
                    .expect("No error");
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
    mut infos: ResMut<Assets<ModelInfo>>,
    mut state: ResMut<State>,
) {
    let mut entities = Vec::new();

    for response in state.processor.response_receiver.try_iter() {
        match response {
            ProcessorResponse::RemoveModelInstances(uuids) => uuids.iter().for_each(|uuid| {
                infos.remove(*uuid);
                materials.remove(*uuid);
            }),
            ProcessorResponse::NewMeshAsset(uuid, mesh) => {
                log::info!("New mesh: {uuid}");
                meshes.insert(uuid, mesh);
            }
            ProcessorResponse::NewModelInfo(uuid, info) => {
                log::info!("New model info: {uuid}");
                materials.insert(uuid, info.get_default_material());
                infos.insert(uuid, info);
            }
            ProcessorResponse::SpawnModelInstances(uuids) => {
                entities.extend(uuids.iter().filter_map(|uuid| {
                    log::info!("Spawn model: {uuid}");

                    infos.get(*uuid).map(|info| {
                        commands
                            .spawn((
                                Mesh3d(Handle::Weak(bevy::asset::AssetId::<Mesh>::Uuid {
                                    uuid: info.geometry_output_uuid,
                                })),
                                MeshMaterial3d(Handle::Weak(bevy::asset::AssetId::<
                                    StandardMaterial,
                                >::Uuid {
                                    uuid: *uuid,
                                })),
                                info.transform,
                                info.get_outline(),
                                OutlineMode::FloodFlat,
                                info.clone(),
                            ))
                            .id()
                    })
                }))
            }
        }

        if state.processor.response_receiver.is_empty() {
            break;
        }
    }

    if !entities.is_empty() {
        // Despawn all entities to remove them from the scene
        for entity in &state.scene.model_entities {
            commands.entity(*entity).despawn();
        }
        state.scene.model_entities = entities;
    }
}

/// A system that draws hit indicators for every pointer.
pub fn model_info_under_cursor(
    pointers: Query<&bevy::picking::pointer::PointerInteraction>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(
        &ModelInfo,
        &mut MeshMaterial3d<StandardMaterial>,
        &mut OutlineVolume,
    )>,
    _assets: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, hit) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
    {
        match query.get_mut(*entity) {
            Ok((model_info, ref mut _material, ref mut outline)) => {
                if buttons.just_pressed(MouseButton::Left) {
                    //let material = assets.get(material.id()).expect("Material");

                    outline.visible = !outline.visible;
                    log::info!(
                        "Model info {} @ {}",
                        model_info.model_hash,
                        hit.position.unwrap()
                    );
                }
            }
            Err(err) => {
                log::error!("{err}");
            }
        }
    }
}
