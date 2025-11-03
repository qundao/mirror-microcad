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

pub mod bevy_types {
    pub use bevy::prelude::{AlphaMode, TypePath, Vec3};
    pub use bevy::{
        asset::Asset,
        pbr::Material,
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
