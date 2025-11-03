// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Viewer materials.

use bevy::{
    app::{App, Plugin, Startup},
    asset::{Assets, Handle, uuid::Uuid},
    ecs::system::ResMut,
    render::render_resource::{Shader, ShaderRef},
};

mod angle;
mod grid;
mod ruler;

pub use angle::Angle;
pub use grid::Grid;
pub use ruler::Ruler;

use crate::to_bevy;

pub mod bevy_types {
    pub use bevy::prelude::{AlphaMode, TypePath, Vec3};
    pub use bevy::{
        asset::Asset,
        pbr::{Material, StandardMaterial},
        render::render_resource::{AsBindGroup, ShaderRef},
    };
}

pub static BUILTIN_MATERIAL_ASSET_ID: u64 = 0x1234123412341234;

fn asset_uuid_from_str(s: &'static str) -> Uuid {
    use std::hash::{Hash, Hasher};
    let mut hasher = rustc_hash::FxHasher::default();
    s.hash(&mut hasher);
    Uuid::from_u64_pair(BUILTIN_MATERIAL_ASSET_ID, hasher.finish())
}

pub fn shader_ref_from_str(s: &'static str) -> ShaderRef {
    ShaderRef::Handle(Handle::Weak(bevy::asset::AssetId::<Shader>::Uuid {
        uuid: asset_uuid_from_str(s),
    }))
}

/// Get correct alpha mode for colors.
pub fn alpha_mode_for_color(color: &microcad_core::Color) -> bevy_types::AlphaMode {
    if color.a >= 1.0 {
        bevy_types::AlphaMode::Opaque
    } else {
        bevy_types::AlphaMode::Blend
    }
}

/// Create a 2D material (unlit) from render attributes.
pub fn create_2d_material(color: &microcad_core::Color) -> bevy_types::StandardMaterial {
    bevy_types::StandardMaterial {
        base_color: to_bevy::color(color),
        alpha_mode: alpha_mode_for_color(color),
        unlit: true,
        double_sided: true,
        ..Default::default()
    }
}

/// Create a 3D material (lit) from a color.
pub fn create_3d_material(color: &microcad_core::Color) -> bevy_types::StandardMaterial {
    bevy_types::StandardMaterial {
        base_color: to_bevy::color(color),
        metallic: 0.1,
        alpha_mode: alpha_mode_for_color(color),
        unlit: false,
        perceptual_roughness: 0.1,
        reflectance: 0.4,
        ..Default::default()
    }
}

pub struct MaterialPlugin;

impl Plugin for MaterialPlugin {
    fn build(&self, app: &mut App) {
        type Material<T> = bevy::prelude::MaterialPlugin<T>;
        app.add_plugins(Material::<Grid>::default())
            .add_plugins(Material::<Angle>::default())
            .add_plugins(Material::<Ruler>::default())
            .add_systems(Startup, load_materials);
    }
}

macro_rules! add_shader_asset {
    ($shaders:ident, $s:literal) => {
        $shaders.insert(
            asset_uuid_from_str($s),
            Shader::from_wgsl(include_str!($s), $s.to_string()),
        );
    };
}

fn load_materials(mut shaders: ResMut<Assets<Shader>>) {
    add_shader_asset!(shaders, "angle.wgsl");
    add_shader_asset!(shaders, "arrow.wgsl");
    add_shader_asset!(shaders, "grid.wgsl");
    add_shader_asset!(shaders, "ruler.wgsl");
}
