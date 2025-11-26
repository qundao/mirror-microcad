// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, ResMut},
    },
    math::Vec3,
    pbr::{DirectionalLight, PointLight, light_consts},
    transform::components::Transform,
};

use crate::ToBevy;

/// Setup lights
pub fn spawn_lights(mut commands: Commands, mut state: ResMut<crate::state::State>) {
    for entity in &state.scene.light_entities {
        commands.entity(*entity).despawn();
    }
    let mut entities: Vec<Entity> = Vec::new();
    let radius = state.scene.radius;

    entities.push(
        commands
            .spawn((
                DirectionalLight {
                    illuminance: light_consts::lux::OVERCAST_DAY,
                    ..Default::default()
                },
                Transform::from_xyz(2.0 * radius, -radius, 3.0 * radius)
                    .looking_at(Vec3::ZERO, Vec3::Z),
            ))
            .id(),
    );
    entities.push(
        commands
            .spawn((
                DirectionalLight {
                    illuminance: light_consts::lux::OVERCAST_DAY,
                    ..Default::default()
                },
                Transform::from_xyz(-2.0 * radius, radius, -3.0 * radius)
                    .looking_at(Vec3::ZERO, Vec3::Z),
            ))
            .id(),
    );
    entities.push(
        commands
            .spawn((
                PointLight {
                    color: state.config.theme.brighter.to_bevy(),
                    range: radius * 100.0,
                    radius: radius / 10.0,
                    intensity: 500000000.0,
                    ..Default::default()
                },
                Transform::from_xyz(2.0 * radius, -2.0 * radius, 2.0 * radius),
            ))
            .id(),
    );
    entities.push(
        commands
            .spawn((
                PointLight {
                    color: state.config.theme.signal.to_bevy(),
                    range: radius * 100.0,
                    radius: radius / 10.0,
                    intensity: 10000000.0,
                    ..Default::default()
                },
                Transform::from_xyz(-2.0 * radius, 2.0 * radius, -2.0 * radius),
            ))
            .id(),
    );

    state.scene.light_entities = entities;
}
