// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Angle measure scene item.

use bevy::{
    asset::{Asset, Assets},
    ecs::system::{Commands, ResMut},
    math::{Vec2, Vec3, primitives::Plane3d},
    pbr::{Material, MeshMaterial3d},
    reflect::TypePath,
};
use bevy_render::{
    alpha::AlphaMode,
    mesh::{Mesh, Mesh3d},
    render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Asset, AsBindGroup, Debug, Clone, Default, TypePath)]
// This struct defines the data that will be passed to your shader
pub struct AngleMaterial {
    #[uniform(0)]
    start_angle: f32,

    #[uniform(1)]
    end_angle: f32,

    #[uniform(2)]
    inner_radius: f32,

    #[uniform(3)]
    outer_radius: f32,

    alpha_mode: AlphaMode,
}

impl AngleMaterial {
    pub const SOURCE: &'static str = "shaders/angle.wgsl";
}

impl Material for AngleMaterial {
    fn fragment_shader() -> ShaderRef {
        Self::SOURCE.into()
    }

    fn vertex_shader() -> ShaderRef {
        Self::SOURCE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

pub fn spawn_angle_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<AngleMaterial>>,
) {
    let plane = Mesh::from(Plane3d::new(Vec3::Z, Vec2::new(10.0, 10.0)));
    let mesh_handle = meshes.add(plane);

    use cgmath::{Deg, Rad};

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(AngleMaterial {
            start_angle: Rad::from(Deg(45.0)).0,
            end_angle: Rad::from(Deg(135.0)).0,
            inner_radius: 0.0,
            outer_radius: 1.0,
            alpha_mode: AlphaMode::Blend,
        })),
        bevy::picking::Pickable::IGNORE,
    ));
}
