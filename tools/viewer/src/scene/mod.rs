use bevy::prelude::*;

mod angle;
mod camera;
mod grid;
mod lighting;
mod ruler;

pub use grid::GridMaterial;

use bevy::asset::uuid::Uuid;
use bevy::prelude::{Handle, Shader};
use bevy::render::render_resource::ShaderRef;

pub static INTERNAL_ASSET_ID: u64 = 0x1234123412341234;

pub fn internal_asset_uuid_from_str(s: &'static str) -> Uuid {
    use std::hash::{Hash, Hasher};
    let mut hasher = rustc_hash::FxHasher::default();
    s.hash(&mut hasher);
    Uuid::from_u64_pair(INTERNAL_ASSET_ID, hasher.finish())
}

pub fn shader_ref_from_str(s: &'static str) -> ShaderRef {
    ShaderRef::Handle(Handle::Weak(bevy::asset::AssetId::<Shader>::Uuid {
        uuid: internal_asset_uuid_from_str(s),
    }))
}

pub fn get_current_zoom_level(projection: &Projection) -> f32 {
    match projection {
        Projection::Orthographic(orthographic_projection) => orthographic_projection.scale,
        _ => todo!(),
    }
}

/// A system that draws hit indicators for every pointer.
pub fn draw_mesh_intersections(
    pointers: Query<&bevy::picking::pointer::PointerInteraction>,
    mut gizmos: Gizmos,
    query: Query<&Projection>,
) {
    for (_entity, hit) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
    {
        if let Some(position) = hit.position {
            let zoom = get_current_zoom_level(query.iter().next().unwrap());
            gizmos.sphere(
                position,
                zoom * 0.1,
                bevy::color::palettes::tailwind::RED_500,
            );
        }
    }
}

/// Scene radius.
pub struct Scene {
    /// Radius of scene's bounding sphere.
    pub radius: f32,
    /// The scene grid (will be `Some` after the grid has been spawned).
    pub grid_entity: Option<Entity>,
    /// Light entities.
    pub light_entities: Vec<Entity>,
    /// Model entities.
    pub model_entities: Vec<Entity>,
}

impl Scene {
    pub const MINIMUM_RADIUS: f32 = 10.0;
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            radius: 100.0,
            grid_entity: Default::default(),
            light_entities: Default::default(),
            model_entities: Default::default(),
        }
    }
}

#[derive(Event)]
pub struct SceneRadiusChangeEvent {
    pub new_radius: f32,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<grid::GridMaterial>::default())
            .add_plugins(MaterialPlugin::<angle::AngleMaterial>::default())
            .add_plugins(MaterialPlugin::<ruler::RulerMaterial>::default())
            .add_plugins(camera::camera_controller::CameraControllerPlugin)
            .add_event::<SceneRadiusChangeEvent>()
            .add_systems(Update, lighting::spawn_lights)
            .add_systems(Startup, grid::spawn_grid_plane)
            //.add_systems(Startup, angle::spawn_angle_plane)
            //.add_systems(Startup, ruler::spawn_ruler_plane)
            .add_systems(Startup, camera::setup_camera)
            .add_systems(Update, camera::update_camera_on_scene_change)
            .add_systems(Update, draw_mesh_intersections)
            .add_systems(Update, grid::update_grid)
            .add_systems(Update, grid::update_grid_on_scene_change)
            .add_systems(Update, grid::update_grid_on_view_angle_change);
    }
}
