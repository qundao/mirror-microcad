// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Conversions from microcad types into bevy meshes.

use super::*;

/// Create a mesh from a line string.
pub fn line_string(line_string: &microcad_core::LineString, z: Scalar) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::LineStrip,
        RenderAssetUsages::default(),
    );
    use bevy::prelude::Vec3;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        line_string
            .coords()
            .map(|c| Vec3::new(c.x as f32, c.y as f32, z as f32))
            .collect::<Vec<_>>(),
    );
    mesh
}

/// Create a mesh from a multi line string.
pub fn multi_line_string(multi_line_string: &microcad_core::MultiLineString, z: Scalar) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::LineList,
        RenderAssetUsages::default(),
    );
    use bevy::prelude::Vec3;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        multi_line_string
            .0
            .iter()
            .flat_map(|line_string| {
                line_string
                    .0
                    .as_slice()
                    .windows(2)
                    .flat_map(|c| {
                        c.iter()
                            .map(|c| Vec3::new(c.x as f32, c.y as f32, z as f32))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    mesh
}
