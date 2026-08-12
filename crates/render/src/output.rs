// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render output type.

use std::rc::Rc;

use cgmath::SquareMatrix;
use microcad_core::{
    Bounds2D, Bounds3D, CalcBounds2D, CalcBounds3D, Geometry2D, Geometry3D, Length, Mat4,
    RenderResolution, Vec2, Vec3, WithBounds2D, WithBounds3D,
};
use microcad_hash::{HashId, ToHash, hash_id};
use microcad_lang_types::{ModelRef, model::ModelOutputType};

use crate::{RenderAttributes, RenderResult};

/// Geometry 2D type alias.
pub type Geometry2DOutput = Rc<WithBounds2D<Geometry2D>>;

/// Geometry 3D type alias.
pub type Geometry3DOutput = Rc<WithBounds3D<Geometry3D>>;

/// Geometry output to be stored in the render cache.
#[derive(Debug, Clone, derive_more::From)]
pub enum GeometryOutput {
    /// 2D output.
    Geometry2D(Geometry2DOutput),
    /// 3D output.
    Geometry3D(Geometry3DOutput),
}

impl GeometryOutput {
    /// The radius of a centered circle that wraps the output geometries bounds on the ground.
    pub fn ground_radius(&self) -> Length {
        let mut bounds = match &self {
            GeometryOutput::Geometry2D(geo2d) => geo2d.bounds.clone(),
            GeometryOutput::Geometry3D(geo3d) => {
                Bounds2D::new(geo3d.bounds.min.truncate(), geo3d.bounds.max.truncate())
            }
        };
        bounds.extend_by_point(Vec2::new(0.0, 0.0));
        Length::mm(bounds.radius())
    }

    /// The radius of a centered sphere, that wrap the geometries bounds.
    pub fn scene_radius(&self) -> Length {
        let mut bounds = match &self {
            GeometryOutput::Geometry2D(geo2d) => {
                Bounds3D::new(geo2d.bounds.min.extend(0.0), geo2d.bounds.max.extend(0.0))
            }
            GeometryOutput::Geometry3D(geo3d) => geo3d.bounds.clone(),
        };
        bounds.extend_by_point(Vec3::new(0.0, 0.0, 0.0));
        Length::mm(bounds.radius())
    }
}

impl From<Geometry2D> for GeometryOutput {
    fn from(geo: Geometry2D) -> Self {
        Self::Geometry2D(Rc::new(geo.into()))
    }
}

impl From<Geometry3D> for GeometryOutput {
    fn from(geo: Geometry3D) -> Self {
        Self::Geometry3D(Rc::new(geo.into()))
    }
}

/// The model output when a model has been processed.
#[derive(Debug, Clone)]
pub struct RenderOutput {
    /// The output (2D/3D) this render output is expected to produce.
    pub output_type: ModelOutputType,
    /// Local transformation matrix.
    pub local_matrix: Option<Mat4>,
    /// World transformation matrix.
    pub world_matrix: Option<Mat4>,
    /// The render resolution, calculated from transformation matrix.
    pub resolution: Option<RenderResolution>,
    /// The output geometry.
    pub geometry: Option<GeometryOutput>,
    /// Render attributes.
    pub attributes: RenderAttributes,
    /// Computed model hash.
    hash: HashId,
}

impl RenderOutput {
    /// Create new render output for model.
    pub fn new<'tree>(model: &ModelRef<'tree>) -> RenderResult<Self> {
        let output_type = model.output_type();
        let hash = hash_id!(model);
        let local_matrix = Some(Mat4::identity()); /*
        TODO: Get local matrix transform for element
        model
        .element()
        .get_affine_transform()?
        .map(|affine_transform| affine_transform.mat3d());
         */

        Ok(RenderOutput {
            output_type,
            local_matrix,
            world_matrix: None,
            resolution: None,
            geometry: None,
            attributes: RenderAttributes::default(), // TODO: Get render attributes from model.into(),
            hash,
        })
    }

    /// Set the world matrix for render output.
    pub fn set_world_matrix(&mut self, m: Mat4) {
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

    /// Local matrix.
    pub fn local_matrix(&self) -> Option<Mat4> {
        self.local_matrix
    }

    /// The radius of a centered circle that wraps the output geometries bounds on the ground.
    pub fn ground_radius(&self) -> Length {
        self.geometry
            .as_ref()
            .map(|geo| geo.ground_radius())
            .unwrap_or_default()
    }

    /// The radius of a centered sphere that wraps the output geometries bounds.
    pub fn scene_radius(&self) -> Length {
        self.geometry
            .as_ref()
            .map(|geo| geo.scene_radius())
            .unwrap_or_default()
    }

    pub fn output_type(&self) -> ModelOutputType {
        self.output_type
    }
}

impl std::fmt::Display for RenderOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{output_type} ({hash}): {geo} {resolution}",
            output_type = match self.output_type {
                ModelOutputType::Geometry2D => "2D",
                ModelOutputType::Geometry3D => "3D",
                ModelOutputType::Any => "Any",
                ModelOutputType::NotDetermined => "?",
            },
            hash = self.to_hash(),
            geo = match &self.geometry {
                Some(GeometryOutput::Geometry2D(geo)) => geo.name(),
                Some(GeometryOutput::Geometry3D(geo)) => geo.name(),
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

impl CalcBounds2D for RenderOutput {
    fn calc_bounds_2d(&self) -> Bounds2D {
        match &self.geometry {
            Some(GeometryOutput::Geometry2D(output)) => output.bounds.clone(),
            _ => Bounds2D::default(),
        }
    }
}

impl CalcBounds3D for RenderOutput {
    fn calc_bounds_3d(&self) -> Bounds3D {
        match &self.geometry {
            Some(GeometryOutput::Geometry3D(output)) => output.bounds.clone(),
            _ => Bounds3D::default(),
        }
    }
}
