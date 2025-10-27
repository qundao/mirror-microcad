// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer geometry output.

use bevy::render::{
    alpha::AlphaMode,
    mesh::{Indices, Mesh},
};
use bevy::{
    asset::RenderAssetUsages, math::Vec3, pbr::StandardMaterial, platform::collections::HashMap,
};
use cgmath::InnerSpace;
use microcad_core::*;
use microcad_lang::{
    model::{Model, OutputType},
    render::ComputedHash,
};

/// Converts a TriangleMesh into a Bevy Mesh with smooth normals (within angle threshold),
/// and sharp edges where the angle between adjacent faces exceeds the threshold.
pub fn triangle_mesh_to_bevy_with_smoothness(mesh: &TriangleMesh, threshold_degrees: f32) -> Mesh {
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
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh_out.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
    mesh_out.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
    mesh_out.insert_indices(Indices::U32(new_indices));
    mesh_out
}

/// Create a bevy mesh from a 2D geometry.
pub fn from_geometry2d(geometry: &Geometry2D, z: Scalar) -> Mesh {
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

/// The output geometry from a µcad model that will be passed to Bevy.
///
/// Processing the mesh geometry will spawn bevy commands to eventually add an entity with a mesh, material and other components to a scene.
pub struct OutputGeometry {
    pub model_hash: u64, // It might be useful to have the model hash as reference to a specific model node.
    //pub color: Color, // We may generate a color
    pub output_type: OutputType,
    pub mesh: Mesh,
    pub material: StandardMaterial,
    pub bounding_radius: f32,
}

impl OutputGeometry {
    /// Create [`OutputGeometry`] from µcad model.
    pub fn from_model(model: &Model) -> Option<Self> {
        let model_ = model.borrow();
        let output = model_.output();

        match output {
            microcad_lang::render::RenderOutput::Geometry3D {
                world_matrix,
                geometry,
                ..
            } => {
                let mat = world_matrix.expect("Some matrix");
                match geometry {
                    // Create 3D geometry output (with lighting).
                    Some(geometry) => {
                        let geometry = geometry.transformed_3d(&mat);

                        Some(Self {
                            model_hash: model.computed_hash(),
                            mesh: triangle_mesh_to_bevy_with_smoothness(
                                &geometry.inner.into(),
                                20.0,
                            ),
                            output_type: OutputType::Geometry3D,
                            material: StandardMaterial {
                                base_color: bevy::prelude::Color::srgba(0.5, 0.5, 0.5, 1.0),
                                metallic: 0.5,
                                alpha_mode: AlphaMode::Opaque,
                                unlit: false,
                                ..Default::default()
                            },
                            bounding_radius: geometry.bounds.max.magnitude() as f32,
                        })
                    }
                    None => None,
                }
            }

            microcad_lang::render::RenderOutput::Geometry2D {
                world_matrix,
                geometry,
                ..
            } => {
                let mat = world_matrix.expect("Some matrix");

                match geometry {
                    // Create 2D geometry output (without lighting).
                    Some(geometry) => {
                        let geometry = geometry.transformed_2d(&mat);

                        Some(Self {
                            model_hash: model.computed_hash(),
                            mesh: from_geometry2d(&geometry.inner, 0.0),
                            output_type: OutputType::Geometry2D,
                            material: StandardMaterial {
                                base_color: bevy::prelude::Color::srgba(0.5, 0.5, 0.5, 1.0),
                                alpha_mode: AlphaMode::Opaque,
                                unlit: true,
                                ..Default::default()
                            },
                            bounding_radius: geometry.bounds.max.magnitude() as f32,
                        })
                    }
                    None => None,
                }
            }
        }
    }
}
