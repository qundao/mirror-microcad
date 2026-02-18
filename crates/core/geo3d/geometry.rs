// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    traits::{Center, TotalMemory, VertexCount},
    *,
};

use derive_more::From;
use std::rc::Rc;
use strum::IntoStaticStr;

use crate::geo3d::*;

/// 3D Geometry
#[derive(IntoStaticStr, From, Clone)]
pub enum Geometry3D {
    /// Triangle mesh.
    Mesh(TriangleMesh),
    /// Manifold.
    Manifold(Rc<Manifold>),
    /// Collection.
    Collection(Geometries3D),
}

impl std::fmt::Debug for Geometry3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")
    }
}

impl Geometry3D {
    /// Return name of geometry.
    pub fn name(&self) -> &'static str {
        self.into()
    }

    /// Execute boolean operation.
    pub fn boolean_op(&self, other: &Geometry3D, op: &BooleanOp) -> Option<Self> {
        let op: manifold_rs::BooleanOp = op.into();
        let a: Rc<Manifold> = self.clone().into();
        let b: Rc<Manifold> = other.clone().into();
        Some(Geometry3D::Manifold(Rc::new(a.boolean_op(&b, op))))
    }

    /// Calculate contex hull.
    pub fn hull(&self) -> Self {
        match &self {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh.to_manifold().hull().into(),
            Geometry3D::Manifold(manifold) => manifold.hull().into(),
            Geometry3D::Collection(collection) => {
                TriangleMesh::from(collection).to_manifold().hull().into()
            }
        }
    }

    /// Return this geometry with calculated bounds.
    pub fn with_bounds(self) -> WithBounds3D<Geometry3D> {
        let bounds = self.calc_bounds_3d();
        WithBounds3D {
            bounds,
            inner: self,
        }
    }
}

impl CalcBounds3D for Geometry3D {
    fn calc_bounds_3d(&self) -> Bounds3D {
        match self {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh.calc_bounds_3d(),
            Geometry3D::Manifold(manifold) => {
                TriangleMesh::from(manifold.to_mesh()).calc_bounds_3d()
            }
            Geometry3D::Collection(collection) => collection.calc_bounds_3d(),
        }
    }
}

impl Transformed3D for Geometry3D {
    fn transformed_3d(&self, mat: &Mat4) -> Self {
        TriangleMesh::from(self.clone()).transformed_3d(mat).into()
    }
}

impl Center for Geometry3D {
    fn center(&self) -> Self {
        let d: Vec3 = self.calc_bounds_3d().center();
        self.transformed_3d(&Mat4::from_translation(-d))
    }
}

impl From<Manifold> for Geometry3D {
    fn from(manifold: Manifold) -> Self {
        Geometry3D::Manifold(Rc::new(manifold))
    }
}

impl From<Geometry3D> for Rc<Manifold> {
    fn from(geo: Geometry3D) -> Self {
        match geo {
            Geometry3D::Mesh(triangle_mesh) => Rc::new(triangle_mesh.to_manifold()),
            Geometry3D::Manifold(manifold) => manifold,
            Geometry3D::Collection(ref collection) => {
                Rc::new(TriangleMesh::from(collection).to_manifold())
            }
        }
    }
}

impl TotalMemory for Rc<Manifold> {} // TODO: Get estimation of total memory of Manifold via C++ API.

impl TotalMemory for Geometry3D {
    fn heap_memory(&self) -> usize {
        match &self {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh.heap_memory(),
            Geometry3D::Manifold(manifold) => manifold.heap_memory(),
            Geometry3D::Collection(collection) => collection.heap_memory(),
        }
    }
}

impl VertexCount for Rc<Manifold> {
    fn vertex_count(&self) -> usize {
        0 // TODO: Get number of vertices for Manifold via C++ API
    }
}

impl VertexCount for Geometry3D {
    fn vertex_count(&self) -> usize {
        match &self {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh.vertex_count(),
            Geometry3D::Manifold(manifold) => manifold.vertex_count(),
            Geometry3D::Collection(collection) => collection.vertex_count(),
        }
    }
}
