pub mod camera_controller;

use bevy::prelude::*;

use crate::State;

pub fn setup_camera(mut commands: Commands, state: Res<State>) {
    let radius = state.scene.radius;

    // Place the camera on the +X, +Y, +Z diagonal of the sphere
    let direction = bevy::prelude::Vec3::new(1.0, 1.0, 1.0).normalize() * radius;

    // camera
    commands.spawn((
        Camera3d::default(),
        camera_controller::CameraController::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: radius,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(direction)
            .looking_at(bevy::prelude::Vec3::ZERO, bevy::prelude::Vec3::Z),
    ));
}

pub fn update_camera_on_scene_change(
    mut events: EventReader<super::SceneRadiusChangeEvent>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for event in events.read() {
        for mut transform in query.iter_mut() {
            use bevy::prelude::Vec3;
            // Assume the camera is looking at the origin
            let current_direction = (transform.translation - Vec3::ZERO).normalize();
            let new_position = current_direction * event.new_radius;

            *transform = Transform::from_translation(new_position).looking_at(Vec3::ZERO, Vec3::Z);

            /*let window = windows.iter().next().unwrap();

            match projection.as_mut() {
                Projection::Orthographic(ortho) => {
                    ortho.near = event.new_radius / 100000.0;
                    ortho.far = event.new_radius * 10.0;
                    /*ortho.scaling_mode = bevy_render::camera::ScalingMode::FixedVertical {
                        viewport_height: event.new_radius,
                    };*/
                    ortho.update(window.width(), window.height());
                    projection.update(window.width(), window.height());
                }
                _ => todo!(),
            };*/
        }
    }
}
