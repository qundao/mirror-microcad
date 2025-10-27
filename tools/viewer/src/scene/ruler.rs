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
pub struct RulerMaterial {
    #[uniform(0)]
    zoom_level: f32,

    alpha_mode: AlphaMode,
}

impl RulerMaterial {
    pub const SOURCE: &'static str = "shaders/ruler.wgsl";
}

impl Material for RulerMaterial {
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

pub fn spawn_ruler_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RulerMaterial>>,
) {
    let plane = Mesh::from(Plane3d::new(Vec3::Z, Vec2::new(10.0, 2.0)));
    let mesh_handle = meshes.add(plane);

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(RulerMaterial {
            zoom_level: 1.0,
            alpha_mode: AlphaMode::Blend,
        })),
        bevy::picking::Pickable::IGNORE,
    ));
}
