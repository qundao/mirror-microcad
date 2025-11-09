// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Viewer materials.

use bevy::render::{
    camera::{Camera, Projection},
    mesh::{Mesh, Mesh3d},
};
use bevy::{
    asset::Assets,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3, primitives::Plane3d},
    pbr::MeshMaterial3d,
    transform::components::Transform,
};

use crate::{scene::get_current_zoom_level, state::State};
use crate::{to_bevy::ToBevy, *};

pub fn spawn_grid_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<material::Grid>>,
    mut state: ResMut<State>,
) {
    let plane = Mesh::from(Plane3d::new(Vec3::Z, Vec2::new(1000.0, 1000.0)));
    let mesh_handle = meshes.add(plane);

    let theme = &state.config.theme;

    state.scene.grid_entity = Some(
        commands
            .spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(materials.add(material::Grid {
                    grid_color: theme.darker.to_bevy(),
                    x_axis_color: theme.bright.to_bevy(),
                    y_axis_color: theme.bright.to_bevy(),
                    ..Default::default()
                })),
                bevy::picking::Pickable::IGNORE,
            ))
            .id(),
    );
}

/// Update grid material according to zoom level.
pub fn update_grid(
    mut materials: ResMut<Assets<material::Grid>>,
    state: Res<State>,
    proj_query: Query<&Projection, With<Camera>>,
    mat_query: Query<&mut MeshMaterial3d<material::Grid>>,
) {
    let radius = state.scene.radius;

    for projection in proj_query {
        if let Some(grid) = state.scene.grid_entity
            && let Ok(material) = mat_query.get(grid)
            && let Some(material) = materials.get_mut(material)
        {
            material.radius = radius;
            material.zoom_level = 1.0 / get_current_zoom_level(projection);
        }
    }
}

pub fn update_grid_on_view_angle_change(
    mut materials: ResMut<Assets<material::Grid>>,
    state: Res<State>,
    cam_query: Query<(&Transform, &Camera)>,
    mat_query: Query<&mut MeshMaterial3d<material::Grid>>,
) {
    for (transform, _) in cam_query {
        if let Some(grid) = state.scene.grid_entity
            && let Ok(material) = mat_query.get(grid)
            && let Some(material) = materials.get_mut(material)
        {
            material.view_angle = transform.forward().normalize();
        }
    }
}
