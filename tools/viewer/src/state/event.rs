// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad State event.

use bevy::{
    asset::{Assets, uuid::Uuid},
    ecs::{
        event::{Event, EventReader},
        system::{Query, ResMut},
    },
    pbr::MeshMaterial3d,
};
use microcad_core::Length;

use crate::{
    State, material,
    state::{Cursor, ModelViewState},
};

/// An event that is fired when the state is
#[derive(Event)]
pub enum StateEvent {
    ChangeGroundRadius(Length),
    SelectAll,
    ClearSelection,
    SelectOne(Uuid),
    SetCursor(Cursor),
}

pub fn handle_state_event(
    mut materials: ResMut<Assets<material::Grid>>,
    mat_query: Query<&mut MeshMaterial3d<material::Grid>>,

    mut state: ResMut<State>,
    mut assets: ResMut<Assets<ModelViewState>>,
    mut events: EventReader<StateEvent>,
) {
    for event in events.read() {
        match event {
            StateEvent::ChangeGroundRadius(radius) => {
                if let Some(grid) = state.scene.grid_entity
                    && let Ok(material) = mat_query.get(grid)
                    && let Some(material) = materials.get_mut(material)
                {
                    state.scene.radius = **radius as f32;
                    material.radius = **radius as f32;
                    log::info!("Radius: {}", **radius);
                }
            }
            StateEvent::SelectAll => todo!(),
            StateEvent::ClearSelection => todo!(),
            StateEvent::SelectOne(uuid) => todo!(),
            StateEvent::SetCursor(cursor) => todo!(),
        }
    }
}
