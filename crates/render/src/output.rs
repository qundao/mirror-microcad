// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render output type.

use std::sync::Arc;

use cgmath::SquareMatrix;

use microcad_core::{self as core, CalcBounds3D, Mat4};

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
pub struct GeometryOutputInner {
    pub geometry: core::Geometry,
    pub bounds: core::Bounds3D,
    pub attributes: RenderAttributes,
}

impl From<core::Geometry> for GeometryOutputInner {
    fn from(geometry: core::Geometry) -> Self {
        let bounds = geometry.calc_bounds_3d();
        Self {
            geometry,
            bounds,
            attributes: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeometryOutput(pub(crate) Arc<GeometryOutputInner>);

impl From<core::Geometry> for GeometryOutput {
    fn from(geo: core::Geometry) -> Self {
        Self(Arc::new(GeometryOutputInner::from(geo)))
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
        let mut bounds = self.0.bounds.clone();
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
    pub outputs: Vec<GeometryOutput>,
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
