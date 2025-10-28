// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Conversions from microcad core types to bevy types.

use bevy::{
    asset::RenderAssetUsages, math::Vec3, pbr::StandardMaterial, platform::collections::HashMap,
};
use bevy::{
    render::{
        alpha::AlphaMode,
        mesh::{Indices, Mesh},
    },
    transform::components::Transform,
};
use microcad_core::*;

/// Converts a TriangleMesh into a Bevy Mesh with smooth normals (within angle threshold),
/// and sharp edges where the angle between adjacent faces exceeds the threshold.
pub fn mesh_with_smoothness(mesh: &TriangleMesh, threshold_degrees: f32) -> Mesh {
    let cos_threshold = threshold_degrees.to_radians().cos();
    use cgmath::InnerSpace;

    let mut face_normals = Vec::with_capacity(mesh.triangle_indices.len());
    let mut vertex_to_faces: Vec<Vec<usize>> = vec![vec![]; mesh.positions.len()];
    for (face_idx, Triangle(i0, i1, i2)) in mesh.triangle_indices.iter().enumerate() {
        let a = mesh.positions[*i0 as usize];
        let b = mesh.positions[*i1 as usize];
        let c = mesh.positions[*i2 as usize];

        let normal = (b - a).cross(c - a).normalize();
        face_normals.push(normal);

        for &i in [i0, i1, i2] {
            vertex_to_faces[i as usize].push(face_idx);
        }
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct VertexKey {
        index: usize,
        quant_normal: [i16; 3], // Normal, quantized
    }

    // Quantize normal into hashable form
    fn quantize_normal(n: Vec3) -> [i16; 3] {
        [
            (n.x.clamp(-1.0, 1.0) * 32767.0) as i16,
            (n.y.clamp(-1.0, 1.0) * 32767.0) as i16,
            (n.z.clamp(-1.0, 1.0) * 32767.0) as i16,
        ]
    }

    let mut vertex_cache: HashMap<VertexKey, u32> = HashMap::new();
    let mut new_positions = Vec::new();
    let mut new_normals = Vec::new();
    let mut new_indices = Vec::new();

    for (face_idx, Triangle(i0, i1, i2)) in mesh.triangle_indices.iter().enumerate() {
        for &orig_idx in [i0, i1, i2] {
            let pos = mesh.positions[orig_idx as usize];

            // Average normals of adjacent faces within the angle threshold
            let mut normal_sum = Vec3::ZERO;
            for &adj_face_idx in &vertex_to_faces[orig_idx as usize] {
                let adj_normal = face_normals[adj_face_idx];
                if face_normals[face_idx].dot(adj_normal) >= cos_threshold {
                    normal_sum += Vec3::new(adj_normal.x, adj_normal.y, adj_normal.z);
                }
            }

            let smooth_normal = normal_sum.normalize_or_zero();
            let key = VertexKey {
                index: orig_idx as usize,
                quant_normal: quantize_normal(smooth_normal),
            };

            new_indices.push(match vertex_cache.get(&key) {
                Some(idx) => *idx,
                None => {
                    let idx = new_positions.len() as u32;
                    new_positions.push([pos.x, pos.y, pos.z]);
                    new_normals.push(smooth_normal);
                    vertex_cache.insert(key, idx);
                    idx
                }
            });
        }
    }

    let mut mesh_out = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh_out.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
    mesh_out.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
    mesh_out.insert_indices(Indices::U32(new_indices));
    mesh_out
}

/// Create a bevy mesh from a 2D geometry.
pub fn geometry2d(geometry: &Geometry2D, z: Scalar) -> Mesh {
    let multi_polygon = geometry.to_multi_polygon();
    use geo::TriangulateEarcut;

    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for poly in &multi_polygon.0 {
        let triangulation = poly.earcut_triangles_raw();
        let n = positions.len();
        positions.append(
            &mut triangulation
                .vertices
                .as_slice()
                .chunks_exact(2)
                .map(|chunk| [chunk[0] as f32, chunk[1] as f32, z as f32])
                .collect(),
        );

        indices.append(
            &mut triangulation
                .triangle_indices
                .iter()
                .map(|i| (i + n) as u32)
                .collect(),
        );
    }

    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

pub fn mat4(m: cgmath::Matrix4<f64>) -> bevy::prelude::Mat4 {
    use cgmath::Matrix;

    // cgmath stores as column-major, same as glam/Bevy
    let m = m.transpose(); // optional if you’re unsure about order
    bevy::prelude::Mat4::from_cols_array(&[
        m.x.x as f32,
        m.x.y as f32,
        m.x.z as f32,
        m.x.w as f32,
        m.y.x as f32,
        m.y.y as f32,
        m.y.z as f32,
        m.y.w as f32,
        m.z.x as f32,
        m.z.y as f32,
        m.z.z as f32,
        m.z.w as f32,
        m.w.x as f32,
        m.w.y as f32,
        m.w.z as f32,
        m.w.w as f32,
    ])
}

pub fn color(color: microcad_core::Color) -> bevy::prelude::Color {
    bevy::prelude::Color::srgba(color.r, color.g, color.b, color.a)
}

pub fn material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: self::color(color),
        alpha_mode: AlphaMode::Opaque,
        unlit: true,
        ..Default::default()
    }
}

pub fn transform(mat: Mat4) -> Transform {
    Transform::from_matrix(mat4(mat))
}

pub fn bounds_2d(bounds: Bounds2D) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::LineStrip,
        RenderAssetUsages::default(),
    );
    use bevy::prelude::{Vec2, Vec3};
    let min = Vec2::new(bounds.min.x as f32, bounds.min.y as f32);
    let max = Vec2::new(bounds.max.x as f32, bounds.max.y as f32);
    let z = 0.0_f32;
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        [
            Vec3::new(min.x, min.y, z),
            Vec3::new(max.x, min.y, z),
            Vec3::new(max.x, min.y, z),
            Vec3::new(min.x, min.y, z),
            Vec3::new(min.x, min.y, z),
        ]
        .iter()
        .map(|p| [p.x, p.y, p.z])
        .collect::<Vec<_>>(),
    );
    mesh
}

pub fn bounds_3d(bounds: Bounds3D) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::LineStrip,
        RenderAssetUsages::default(),
    );
    use bevy::prelude::Vec3;
    let min = Vec3::new(
        bounds.min.x as f32,
        bounds.min.y as f32,
        bounds.min.z as f32,
    );
    let max = Vec3::new(
        bounds.max.x as f32,
        bounds.max.y as f32,
        bounds.max.z as f32,
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        [
            // Bottom face
            Vec3::new(min.x, min.y, min.z),
            Vec3::new(max.x, min.y, min.z),
            Vec3::new(max.x, min.y, max.z),
            Vec3::new(min.x, min.y, max.z),
            Vec3::new(min.x, min.y, min.z), // close bottom loop
            // Connect to top face
            Vec3::new(min.x, max.y, min.z),
            // Top face
            Vec3::new(max.x, max.y, min.z),
            Vec3::new(max.x, max.y, max.z),
            Vec3::new(min.x, max.y, max.z),
            Vec3::new(min.x, max.y, min.z), // close top loop
            // Back down to start
            Vec3::new(max.x, max.y, min.z),
            Vec3::new(max.x, min.y, min.z),
            Vec3::new(max.x, min.y, max.z),
            Vec3::new(max.x, max.y, max.z),
            Vec3::new(min.x, max.y, max.z),
            Vec3::new(min.x, min.y, max.z),
        ]
        .iter()
        .map(|p| [p.x, p.y, p.z])
        .collect::<Vec<_>>(),
    );
    mesh
}
