use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, ResMut},
    },
    math::Vec3,
    pbr::{DirectionalLight, PointLight, light_consts},
    transform::components::Transform,
};

/// Setup lights
pub fn spawn_lights(mut commands: Commands, mut state: ResMut<crate::state::State>) {
    // Despawn all light entities to remove them from the scene
    for entity in &state.scene.light_entities {
        commands.entity(*entity).despawn();
    }

    let mut entities: Vec<Entity> = Vec::new();
    let radius = state.scene.radius;

    entities.push(
        commands
            .spawn((
                DirectionalLight {
                    shadows_enabled: true,
                    illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
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
                PointLight {
                    intensity: 10_000_000.0,
                    range: radius * 10.0,
                    radius: radius * 2.0,
                    shadows_enabled: false,
                    ..Default::default()
                },
                Transform::from_xyz(0.0, 0.0, radius * 4.0),
            ))
            .id(),
    );

    state.scene.light_entities = entities;
}
