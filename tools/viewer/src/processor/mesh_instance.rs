// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mesh/Material/Transform Bundle converting µcad types into bevy types.

use crate::to_bevy;

use bevy::prelude::{Mesh, StandardMaterial, Transform};

use microcad_core::*;

/// A mesh instance is a bundle of mesh, material, and transform.
///
/// This struct contains everything needed to represent a single renderable
/// object in the scene: its geometry (`mesh`), appearance (`material`),
/// and spatial transformation (`transform`).
///
/// Note: *[`MeshInstance`]s are supposed to created from µcad core types*
pub struct MeshInstance<MATERIAL = StandardMaterial> {
    pub mesh: Mesh,
    pub material: MATERIAL,
    pub transform: Transform,
}

impl MeshInstance {
    /// Create a new bevy mesh instance from µcad 2D geometry.
    pub fn new_2d(geometry: &Geometry2D, color: Color, mat: Mat4) -> Self {
        Self {
            mesh: to_bevy::geometry2d(geometry, 0.0),
            material: to_bevy::material(color),
            transform: to_bevy::transform(mat),
        }
    }

    /// Create a new bevy mesh instance from µcad 3D geometry.
    pub fn new_3d(geometry: &Geometry3D, color: Color, mat: Mat4) -> Self {
        Self {
            mesh: to_bevy::mesh_with_smoothness(&geometry.into(), 20.0),
            material: StandardMaterial {
                base_color: to_bevy::color(color),
                metallic: 0.5,
                alpha_mode: bevy::render::alpha::AlphaMode::Opaque,
                unlit: false,
                ..Default::default()
            },
            transform: to_bevy::transform(mat),
        }
    }

    /// Create a new bevy mesh instance from µcad 2D bounds.
    pub fn new_bounds_2d(bounds: Bounds2D, color: Color, mat: Mat4) -> Self {
        Self {
            mesh: to_bevy::bounds_2d(bounds),
            material: to_bevy::material(color),
            transform: to_bevy::transform(mat),
        }
    }

    /// Create a new bevy mesh instance from µcad 3D bounds.
    pub fn new_bounds_3d(bounds: Bounds3D, color: Color, mat: Mat4) -> Self {
        Self {
            mesh: to_bevy::bounds_3d(bounds),
            material: to_bevy::material(color),
            transform: to_bevy::transform(mat),
        }
    }
}
