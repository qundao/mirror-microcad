// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render output type.

use std::sync::Arc;

use cgmath::SquareMatrix;

use microcad_core::{self as core, CalcBounds3D, Geometry3D, GeometryType, Mat4, Transformed3D};

use microcad_hash::{HashId, ToHash, hash_id};
use microcad_lang_types::{
    ModelNodeRef, ModelTree,
    math::IntoFloat,
    model::{ModelNodeId, ModelType},
};

use crate::{RenderAttributes, RenderResolution};

/// Geometry output to be stored in the render cache.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct GeometryOutput {
    pub id: Option<String>,
    pub attr: RenderAttributes,
    pub geometry: core::Geometry,
    pub bounds: core::Bounds3D,
}

/// Builder methods.
impl GeometryOutput {
    pub fn new(geometry: impl Into<core::Geometry>) -> Self {
        let geometry = geometry.into();
        let bounds = geometry.calc_bounds_3d();
        Self {
            id: None,
            attr: Default::default(),
            geometry,
            bounds,
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_attr(mut self, attr: RenderAttributes) -> Self {
        self.attr = attr;
        self
    }
}

/// Accessors.
impl GeometryOutput {
    /// Return GeometryType.
    pub fn ty(&self) -> GeometryType {
        self.geometry.ty()
    }
}

impl From<core::Geometry> for GeometryOutput {
    fn from(geometry: core::Geometry) -> Self {
        let bounds = geometry.calc_bounds_3d();
        Self {
            id: None,
            attr: Default::default(),
            geometry,
            bounds,
        }
    }
}

impl GeometryOutputs {
    /// Returns the number of geometry outputs.
    #[inline]
    pub fn len(&self) -> usize {
        self.outputs.len()
    }

    /// Returns `true` if there are no geometry outputs.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.outputs.is_empty()
    }
}

impl From<Arc<GeometryOutput>> for GeometryOutputs {
    fn from(geometry: Arc<GeometryOutput>) -> Self {
        let geometry_type = geometry.ty();
        Self {
            outputs: vec![geometry],
            geometry_type,
        }
    }
}

impl std::fmt::Display for GeometryOutputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if matches!(self.geometry_type, GeometryType::Empty) {
            write!(f, "[]")
        } else {
            write!(f, "{type}[{len}]", type = self.geometry_type, len = self.len())
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GeometryOutputs {
    outputs: Vec<Arc<GeometryOutput>>,
    geometry_type: GeometryType,
}

impl GeometryOutputs {
    /// Returns an iterator over references to the output items.
    pub fn iter(&self) -> std::slice::Iter<'_, Arc<GeometryOutput>> {
        self.outputs.iter()
    }
}

// Implement IntoIterator for &GeometryOutputs so callers can write `for output in &outputs`
impl<'a> IntoIterator for &'a GeometryOutputs {
    type Item = &'a Arc<GeometryOutput>;
    type IntoIter = std::slice::Iter<'a, Arc<GeometryOutput>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<Arc<GeometryOutput>> for GeometryOutputs {
    fn from_iter<T: IntoIterator<Item = Arc<GeometryOutput>>>(iter: T) -> Self {
        let outputs: Vec<_> = iter.into_iter().collect();
        let geometry_type = outputs
            .iter()
            .fold(GeometryType::Empty, |acc, item| acc.merge(item.ty()));

        Self {
            outputs,
            geometry_type,
        }
    }
}

impl<T> From<core::WithBounds3D<T>> for GeometryOutput
where
    T: Into<Geometry3D> + CalcBounds3D + Transformed3D,
{
    fn from(with_bounds: core::WithBounds3D<T>) -> Self {
        GeometryOutput {
            id: None,
            geometry: core::Geometry::from(with_bounds.inner.into()),
            bounds: with_bounds.bounds,
            attr: Default::default(),
        }
    }
}

impl GeometryOutput {
    /// The radius of a centered circle that wraps the output geometries bounds on the ground.
    pub fn ground_radius(&self) -> core::Length {
        todo!()
        /*
        let mut bounds = match &self {
            GeometryOutput::Geometry2D(geo2d) => geo2d.bounds.clone(),
            GeometryOutput::Geometry3D(geo3d) => {
            }
        };
        let bounds = Bounds2D::new(geo3d.bounds.min.truncate(), geo3d.bounds.max.truncate());
        bounds.extend_by_point(core::Vec2::new(0.0, 0.0));
        core::Length::mm(bounds.radius())
        */
    }

    /// The radius of a centered sphere, that wrap the geometries bounds.
    pub fn scene_radius(&self) -> core::Length {
        let mut bounds = self.bounds.clone();
        bounds.extend_by_point(core::Vec3::new(0.0, 0.0, 0.0));
        core::Length::mm(bounds.radius())
    }
}

#[derive(Debug, Clone)]
pub struct GeometryNodeData {
    /// The output (2D/3D) this render output is expected to produce.
    pub output_type: ModelType,
    /// Local transformation matrix.
    pub local_matrix: core::Mat4,
    /// World transformation matrix.
    pub world_matrix: core::Mat4,
    /// The render resolution, calculated from transformation matrix.
    pub resolution: Option<RenderResolution>,
    /// The output geometry.
    pub outputs: GeometryOutputs,
    /// Computed model hash.
    hash: HashId,

    pub model_node_id: ModelNodeId,
}

impl GeometryNodeData {
    /// Create new render output for model.
    pub fn new<'tree>(model: ModelNodeRef<'tree>) -> Self {
        let output_type = model.output_type();
        let hash = hash_id!(model);
        let local_matrix = model.local_matrix().into_float();

        GeometryNodeData {
            output_type,
            local_matrix,
            world_matrix: Mat4::identity(),
            resolution: None,
            outputs: Default::default(),
            hash,
            model_node_id: model.id,
        }
    }

    pub fn model<'a>(&self, tree: &'a ModelTree) -> ModelNodeRef<'a> {
        ModelNodeRef::new(self.model_node_id, &tree.arena)
    }

    /// Get render resolution.
    pub fn resolution(&self) -> &Option<RenderResolution> {
        &self.resolution
    }
}

impl ToHash for GeometryNodeData {
    fn to_hash(&self) -> HashId {
        self.hash
    }
}
