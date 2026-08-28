// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render output type.

use std::sync::Arc;

use cgmath::SquareMatrix;

use microcad_core::{self as core, CalcBounds3D};

use microcad_hash::{HashId, ToHash, hash_id};
use microcad_lang_types::{
    ModelNodeRef,
    model::{ModelType, NodeId},
};

use crate::{RenderAttributes, RenderResolution, RenderResult};

/// Geometry output to be stored in the render cache.
#[non_exhaustive]
#[derive(Debug, Clone, derive_more::From)]
pub struct GeometryOutputInner {
    pub geometry: core::Geometry,
    pub bounds: core::Bounds3D,
}

#[derive(Debug, Clone)]
pub struct GeometryOutput(Arc<GeometryOutputInner>);

impl From<core::Geometry> for GeometryOutput {
    fn from(geometry: core::Geometry) -> Self {
        let bounds = geometry.calc_bounds_3d();
        Self(Arc::new(GeometryOutputInner { geometry, bounds }))
    }
}

impl GeometryOutput {
    pub fn name(&self) -> &'static str {
        todo!()
    }

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

/// The model output when a model has been processed.
#[derive(Debug, Clone)]
pub struct RenderOutput {
    /// The output (2D/3D) this render output was expected to produce.
    pub output_type: ModelType,
    /// Local transformation matrix.
    pub local_matrix: Option<core::Mat4>,
    /// World transformation matrix.
    pub world_matrix: Option<core::Mat4>,
    /// The render resolution, calculated from transformation matrix.
    pub resolution: Option<RenderResolution>,
    /// The output geometry.
    pub geometry: Option<GeometryOutput>,
    /// Render attributes.
    pub attributes: RenderAttributes,
    /// Computed model hash.
    hash: HashId,

    _model_node_id: NodeId,
}

impl RenderOutput {
    /// Create new render output for model.
    pub fn new<'tree>(model: ModelNodeRef<'tree>) -> Self {
        let output_type = model.output_type();
        let hash = hash_id!(model);
        let local_matrix = Some(core::Mat4::identity()); /*
        TODO: Get local matrix transform for element
        model
        .element()
        .get_affine_transform()?
        .map(|affine_transform| affine_transform.mat3d());
         */

        RenderOutput {
            output_type,
            local_matrix,
            world_matrix: None,
            resolution: None,
            geometry: None,
            attributes: RenderAttributes::default(), // TODO: Get render attributes from model.into(),
            hash,
            _model_node_id: model.id,
        }
    }

    /// Set the world matrix for render output.
    pub fn set_world_matrix(&mut self, m: core::Mat4) {
        self.world_matrix = Some(m);
    }

    /// Set the 2D geometry as render output.
    pub fn set_geometry(&mut self, geo: GeometryOutput) {
        self.geometry = Some(geo)
    }

    /// Get render resolution.
    pub fn resolution(&self) -> &Option<RenderResolution> {
        &self.resolution
    }

    /// Set render resolution.
    pub fn set_resolution(&mut self, render_resolution: RenderResolution) {
        self.resolution = Some(render_resolution);
    }

    pub fn output_type(&self) -> ModelType {
        self.output_type
    }
}

impl std::fmt::Display for RenderOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{output_type} ({hash}): {geo} {resolution}",
            output_type = match self.output_type {
                ModelType::Geometry2D => "2D",
                ModelType::Geometry3D => "3D",
                ModelType::Any => "Any",
                ModelType::NotDetermined => "?",
            },
            hash = self.to_hash(),
            geo = match &self.geometry {
                Some(geo) => geo.name(),
                None => "",
            },
            resolution = match &self.resolution {
                Some(resolution) => resolution.to_string(),
                None => "".to_string(),
            },
        )?;
        Ok(())
    }
}

impl ToHash for RenderOutput {
    fn to_hash(&self) -> HashId {
        self.hash
    }
}
