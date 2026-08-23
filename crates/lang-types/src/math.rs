// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mathematical operations.

use crate::{Angle, AngleF, Mat3, Mat3F, Scalar, Vec3F};

use cgmath::{InnerSpace, SquareMatrix};

/// Mathematical operations for any value type.
pub trait MathOps {
    type Output;
    type Error;

    fn abs(&self) -> Result<Self::Output, Self::Error>;
    fn signum(&self) -> Result<Self::Output, Self::Error>;

    fn sin(&self) -> Result<Self::Output, Self::Error>;
    fn cos(&self) -> Result<Self::Output, Self::Error>;
    fn tan(&self) -> Result<Self::Output, Self::Error>;

    // Rounding & Conversions
    fn int(&self) -> Result<Self::Output, Self::Error>;
    fn floor(&self) -> Result<Self::Output, Self::Error>;
    fn ceil(&self) -> Result<Self::Output, Self::Error>;
    fn round(&self) -> Result<Self::Output, Self::Error>;
    fn fract(&self) -> Result<Self::Output, Self::Error>;

    fn sqrt(&self) -> Result<Self::Output, Self::Error>;
}

fn angle_f(x: Angle) -> AngleF {
    cgmath::Rad(x.0.to_num::<f64>())
}

fn mat3f_to_mat3(mat3f: Mat3F) -> Mat3 {
    Mat3::new(
        Scalar::from_num(mat3f.x.x),
        Scalar::from_num(mat3f.x.y),
        Scalar::from_num(mat3f.x.z),
        Scalar::from_num(mat3f.y.x),
        Scalar::from_num(mat3f.y.y),
        Scalar::from_num(mat3f.y.z),
        Scalar::from_num(mat3f.z.x),
        Scalar::from_num(mat3f.z.y),
        Scalar::from_num(mat3f.z.z),
    )
}

pub fn rotate_around_axis(angle: Angle, x: Scalar, y: Scalar, z: Scalar) -> Mat3 {
    let axis = Vec3F::new(x.to_num(), y.to_num(), z.to_num());
    let mat3f = Mat3F::from_axis_angle(axis, angle_f(angle));
    mat3f_to_mat3(mat3f)
}

/// Helper function to return rotation X,Y,Z rotation matrices.
pub fn rotation_matrices_xyz(x: Angle, y: Angle, z: Angle) -> (Mat3F, Mat3F, Mat3F) {
    (
        Mat3F::from_angle_x(angle_f(x)),
        Mat3F::from_angle_y(angle_f(y)),
        Mat3F::from_angle_z(angle_f(z)),
    )
}

pub fn rotate_xyz(x: Angle, y: Angle, z: Angle) -> Mat3 {
    let (x, y, z) = rotation_matrices_xyz(x, y, z);
    mat3f_to_mat3(x * y * z)
}

pub fn rotate_zyx(z: Angle, y: Angle, x: Angle) -> Mat3 {
    let (x, y, z) = rotation_matrices_xyz(x, y, z);
    mat3f_to_mat3(z * y * x)
}

/// Rotation matrix to orient a vector
pub fn orient_to_z(target: Vec3F) -> Mat3F {
    let z_axis = Vec3F::unit_z();
    let target = target.normalize();

    // Handle edge case where target is already Z
    if (target - z_axis).magnitude2() < 1e-6 {
        return Mat3F::identity();
    }

    // Handle 180-degree rotation (target is -Z)
    if (target + z_axis).magnitude2() < 1e-6 {
        // Rotate 180° around any axis perpendicular to Z.
        // For stability, pick X if possible, otherwise Y.
        let perp_axis = if z_axis.cross(Vec3F::unit_x()).magnitude2() > 1e-6 {
            Vec3F::unit_x()
        } else {
            Vec3F::unit_y()
        };
        return Mat3F::from_axis_angle(perp_axis, cgmath::Rad(std::f64::consts::PI));
    }

    // Normal case
    let rotation_axis = z_axis.cross(target).normalize();
    let dot = z_axis.dot(target).clamp(-1.0, 1.0); // avoid NaNs
    let angle = cgmath::Rad(dot.acos());

    Mat3F::from_axis_angle(rotation_axis, angle)
}

pub fn align_vectors_rodrigues(from: Vec3F, to: Vec3F) -> Mat3F {
    let from = from.normalize();
    let to = to.normalize();

    let c = from.dot(to);

    if c > 1.0 - 1e-6 {
        return Mat3F::identity();
    }
    if c < -1.0 + 1e-6 {
        let perp_axis = if from.x.abs() > 0.9 {
            Vec3F::unit_y()
        } else {
            Vec3F::unit_x()
        };
        let axis = from.cross(perp_axis).normalize();
        return Mat3F::from_axis_angle(axis, cgmath::Rad(std::f64::consts::PI));
    }

    let v = from.cross(to);

    // Skew-symmetric cross-product matrix [v]x
    let vx = Mat3F::new(0.0, v.z, -v.y, -v.z, 0.0, v.x, v.y, -v.x, 0.0);

    // R = I + vx + vx^2 * (1 / (1 + c))
    let k = 1.0 / (1.0 + c);
    Mat3F::identity() + vx + (vx * vx) * k
}
