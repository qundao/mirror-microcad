// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Operation trait.

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::{Length, Scalar};

/// Transformation matrix
#[derive(Clone, Display, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum AffineTransform {
    /// Translation.
    #[display("x = {x}, y = {y}, z = {z}")]
    Translation { x: Length, y: Length, z: Length },
    /// Generic rotation.
    // Rotation(Mat3), // TODO impl custom hash here
    /// Scale.
    // Scale(Vec3), // Impl display
    /// Uniform scale.
    UniformScale(Scalar),
}

/// Boolean operations
#[derive(Clone, Copy, Debug, Display, Hash, PartialEq, Serialize, Deserialize)]
pub enum BooleanOp {
    /// Computes the union R = P ∪ Q
    Union,
    /// computes the difference R = P ∖ Q
    Difference,
    /// computes the intersection R = P ∩ Q
    Intersect,
}

impl AffineTransform {
    /*/// Get the 2D transformation matrix
    pub fn mat2d(&self) -> Mat3 {
        match self {
            AffineTransform::Translation(v) => Mat3::from_translation(Vec2::new(v.x, v.y)),
            AffineTransform::Rotation(m) => Mat3::from_cols(
                Vec3::new(m.x.x, m.x.y, 0.0),
                Vec3::new(m.y.x, m.y.y, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            AffineTransform::Scale(v) => Mat3::from_nonuniform_scale(v.x, v.y),
            AffineTransform::UniformScale(s) => Mat3::from_scale(*s),
        }
    }

    /// Get the 3D transformation matrix
    pub fn mat3d(&self) -> Mat4 {
        match self {
            AffineTransform::Translation(v) => Mat4::from_translation(*v),
            AffineTransform::Rotation(a) => Mat4::from_cols(
                a.x.extend(0.0),
                a.y.extend(0.0),
                a.z.extend(0.0),
                Vec3::new(0.0, 0.0, 0.0).extend(1.0),
            ),
            AffineTransform::Scale(v) => Mat4::from_nonuniform_scale(v.x, v.y, v.z),
            AffineTransform::UniformScale(s) => Mat4::from_scale(*s),
        }
    }*/
}
