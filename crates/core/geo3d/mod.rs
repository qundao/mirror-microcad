// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry

mod align;
mod bounds;
mod collection;
mod extrude;
mod geometry;
mod mesh;
mod plane;
mod reflect;
mod triangle;
mod vertex;

pub use align::*;
pub use bounds::*;
pub use collection::*;
pub use extrude::*;
pub use geometry::*;
pub use manifold_rust::manifold::Manifold;
pub use mesh::TriangleMesh;
pub use plane::Plane;
pub use reflect::*;
pub use vertex::Vertex;

use crate::BooleanOp;

impl From<BooleanOp> for manifold_rust::types::OpType {
    fn from(op: BooleanOp) -> Self {
        match op {
            BooleanOp::Union => manifold_rust::types::OpType::Add,
            BooleanOp::Intersect => manifold_rust::types::OpType::Intersect,
            BooleanOp::Subtract => manifold_rust::types::OpType::Subtract,
            _ => unimplemented!(),
        }
    }
}

#[test]
fn test_mesh_volume() {
    let manifold = Manifold::sphere(1.0, 512);
    let mesh = TriangleMesh::from(manifold.get_mesh_gl(0));

    let volume = mesh.volume();
    assert!((volume - 4.0 / 3.0 * std::f64::consts::PI).abs() < 1e-3);
}
