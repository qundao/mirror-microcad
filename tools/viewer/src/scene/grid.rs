use bevy::{
    asset::{Asset, Assets},
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3, primitives::Plane3d},
    pbr::{Material, MeshMaterial3d},
    reflect::TypePath,
    transform::components::Transform,
};
use bevy::{
    ecs::event::EventReader,
    render::{
        alpha::AlphaMode,
        camera::{Camera, Projection},
        mesh::{Mesh, Mesh3d},
        render_resource::{AsBindGroup, ShaderRef},
    },
};

use crate::{scene::get_current_zoom_level, state::State};

#[derive(Asset, AsBindGroup, Debug, Clone, Default, TypePath)]
// This struct defines the data that will be passed to your shader
pub struct GridMaterial {
    #[uniform(0)]
    radius: f32,

    #[uniform(1)]
    zoom_level: f32,

    #[uniform(2)]
    view_angle: Vec3,

    alpha_mode: AlphaMode,
}

impl GridMaterial {
    pub const SOURCE: &'static str = "shaders/grid.wgsl";
}

impl Material for GridMaterial {
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

pub fn spawn_grid_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut state: ResMut<State>,
) {
    let radius = state.scene.radius;
    let plane = Mesh::from(Plane3d::new(Vec3::Z, Vec2::new(1000.0, 1000.0)));
    let mesh_handle = meshes.add(plane);

    state.scene.grid_entity = Some(
        commands
            .spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(materials.add(GridMaterial {
                    radius,
                    zoom_level: 1.0,
                    view_angle: Vec3::new(0.0, 0.0, 1.0),
                    alpha_mode: AlphaMode::Blend,
                })),
                bevy::picking::Pickable::IGNORE,
            ))
            .id(),
    );
}

/// Update grid material according to zoom level.
pub fn update_grid(
    mut materials: ResMut<Assets<GridMaterial>>,
    state: Res<State>,
    proj_query: Query<&Projection, With<Camera>>,
    mat_query: Query<&mut MeshMaterial3d<GridMaterial>>,
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
    mut materials: ResMut<Assets<GridMaterial>>,
    state: Res<State>,
    cam_query: Query<(&Transform, &Camera)>,
    mat_query: Query<&mut MeshMaterial3d<GridMaterial>>,
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

/// Update grid material when scene radius has changed.
pub fn update_grid_on_scene_change(
    mut events: EventReader<super::SceneRadiusChangeEvent>,
    mut materials: ResMut<Assets<GridMaterial>>,
    state: Res<State>,
    mat_query: Query<&mut MeshMaterial3d<GridMaterial>>,
) {
    for event in events.read() {
        if let Some(grid) = state.scene.grid_entity
            && let Ok(material) = mat_query.get(grid)
            && let Some(material) = materials.get_mut(material)
        {
            material.radius = event.new_radius;
        }
    }
}
