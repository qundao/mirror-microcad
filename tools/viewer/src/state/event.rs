// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad State event.

use bevy::{
    asset::{Assets, uuid::Uuid},
    ecs::{
        event::{Event, EventReader},
        query::With,
        system::{Query, ResMut},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
    render::camera::{Camera, Projection},
    window::Window,
};
use microcad_core::Length;

use crate::{
    State, material,
    state::{Cursor, ModelViewState},
};

/// An event that is fired when the state is
#[derive(Event)]
pub enum ViewerEvent {
    ChangeGroundRadius(Length),
    SelectAll,
    ClearSelection,
    SelectOne(Uuid),
    SetCursor(Cursor),
    ZoomToFit,
}

impl ViewerEvent {
    fn for_each_view_state(
        f: impl Fn(&Uuid, &mut ModelViewState),
        view_states: &mut Assets<ModelViewState>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        view_states
            .iter_mut()
            .for_each(|(asset_id, view_state)| match asset_id {
                bevy::asset::AssetId::Uuid { uuid } => {
                    // Generate new material.
                    f(&uuid, view_state);
                    let material = materials.get_mut(uuid).expect("Must have material");
                    *material = view_state.generate_material();
                }
                _ => unreachable!(),
            });
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_viewer_event(
    mut grid_materials: ResMut<Assets<material::Grid>>,
    mat_query: Query<&mut MeshMaterial3d<material::Grid>>,
    windows: Query<&Window>,
    mut projection: Query<&mut Projection, With<Camera>>,
    mut state: ResMut<State>,
    mut view_states: ResMut<Assets<ModelViewState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<ViewerEvent>,
) {
    for event in events.read() {
        match event {
            ViewerEvent::ChangeGroundRadius(radius) => {
                if let Some(grid) = state.scene.grid_entity
                    && let Ok(material) = mat_query.get(grid)
                    && let Some(material) = grid_materials.get_mut(material)
                {
                    state.scene.radius = **radius as f32;
                    material.radius = **radius as f32;
                    log::info!("Radius: {}", **radius);
                }
            }
            ViewerEvent::SelectAll => {
                ViewerEvent::for_each_view_state(
                    |_, view_state| {
                        view_state.is_selected = false;
                    },
                    view_states.as_mut(),
                    materials.as_mut(),
                );
            }
            ViewerEvent::ClearSelection => {
                ViewerEvent::for_each_view_state(
                    |_, view_state| {
                        view_state.is_selected = false;
                    },
                    view_states.as_mut(),
                    materials.as_mut(),
                );
            }
            ViewerEvent::SelectOne(selected_uuid) => {
                ViewerEvent::for_each_view_state(
                    |uuid, view_state| {
                        view_state.is_selected = uuid == selected_uuid;
                    },
                    view_states.as_mut(),
                    materials.as_mut(),
                );
            }
            ViewerEvent::SetCursor(_) => ViewerEvent::for_each_view_state(
                |_, _| todo!(),
                view_states.as_mut(),
                materials.as_mut(),
            ),
            ViewerEvent::ZoomToFit => {
                let Ok(mut projection) = projection.single_mut() else {
                    return;
                };
                let Ok(window) = windows.single() else {
                    return;
                };
                crate::scene::zoom_to_fit(projection.as_mut(), window);
            }
        }
    }
}
