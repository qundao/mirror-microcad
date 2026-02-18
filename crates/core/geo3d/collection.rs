// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry collection

use std::rc::Rc;

use derive_more::{Deref, DerefMut};

use crate::{
    geo3d::{CalcBounds3D, bounds::Bounds3D},
    traits::{DistributeGrid, TotalMemory, VertexCount},
    *,
};

/// 3D geometry collection.
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Geometries3D(Vec<Rc<Geometry3D>>);

impl Geometries3D {
    /// New geometry collection.
    pub fn new(geometries: Vec<Geometry3D>) -> Self {
        Self(geometries.into_iter().map(Rc::new).collect())
    }

    /// Append another geometry collection.
    pub fn append(&mut self, mut geometries: Geometries3D) {
        self.0.append(&mut geometries.0)
    }

    /// Apply boolean operation on collection and render to manifold.
    pub fn boolean_op(&self, op: &BooleanOp) -> Rc<Manifold> {
        let manifold_list: Vec<_> = self
            .0
            .iter()
            // Render each geometry into a multipolygon and filter out empty ones
            .filter_map(|geo| {
                let manifold: Rc<Manifold> = geo.as_ref().clone().into();
                if manifold.is_empty() {
                    None
                } else {
                    Some(manifold)
                }
            })
            .collect();

        if manifold_list.is_empty() {
            return Rc::new(Manifold::empty());
        }

        manifold_list[1..]
            .iter()
            .fold(manifold_list[0].clone(), |acc, other| {
                Rc::new(acc.boolean_op(other, op.into()))
            })
    }
}

impl FromIterator<Rc<Geometry3D>> for Geometries3D {
    fn from_iter<T: IntoIterator<Item = Rc<Geometry3D>>>(iter: T) -> Self {
        Geometries3D(iter.into_iter().collect())
    }
}

impl CalcBounds3D for Geometries3D {
    fn calc_bounds_3d(&self) -> Bounds3D {
        self.0.iter().fold(Bounds3D::default(), |bounds, geometry| {
            bounds.extend(geometry.calc_bounds_3d())
        })
    }
}

impl Transformed3D for Geometries3D {
    fn transformed_3d(&self, mat: &Mat4) -> Self {
        Self(
            self.iter()
                .map(|geometry| Rc::new(geometry.transformed_3d(mat)))
                .collect::<Vec<_>>(),
        )
    }
}

impl DistributeGrid for Geometries3D {
    fn distribute_grid(&self, rect: Rect, rows: Integer, columns: Integer) -> Self {
        Geometries3D(
            GridCells::new(rect, rows, columns)
                .zip(self.0.iter())
                .map(|(cell, geo)| {
                    let bounds = geo.calc_bounds_3d();
                    let center = bounds.center();
                    let cell_center: Vec2 = cell.center().x_y().into();
                    let d = center - cell_center.extend(bounds.min.z + center.z);
                    Rc::new(geo.transformed_3d(&Mat4::from_translation(d)))
                })
                .collect(),
        )
    }
}

impl TotalMemory for Geometries3D {
    fn heap_memory(&self) -> usize {
        self.iter().map(|g| g.heap_memory()).sum()
    }
}

impl VertexCount for Geometries3D {
    fn vertex_count(&self) -> usize {
        self.iter().map(|g| g.vertex_count()).sum()
    }
}
