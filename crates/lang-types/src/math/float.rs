// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Floating point/fixed point conversion handling.
//!
//! All values in µcad are fixed point. Use these traits and their impls to convert them to floating point types and back.
//! The default floating point type `ScalarF` is `f64`:

use crate::{Angle, Mat2, Mat3, Mat4, Scalar, Vec2, Vec3, Vec4};

pub type ScalarF = f64;

pub type AngleF = cgmath::Rad<ScalarF>;

/// 2D floating point vector type.
pub type Vec2F = cgmath::Vector2<ScalarF>;

/// 3D floating point vector type.
pub type Vec3F = cgmath::Vector3<ScalarF>;

/// 4D floating point vector type.
pub type Vec4F = cgmath::Vector4<ScalarF>;

/// Matrix 2x2 floating point type.
pub type Mat2F = cgmath::Matrix2<ScalarF>;

/// Matrix 3x3 floating point type.
pub type Mat3F = cgmath::Matrix3<ScalarF>;

/// Matrix 4x4 floating point type.
pub type Mat4F = cgmath::Matrix4<ScalarF>;

/// Lossy conversion to floating-point representation (e.g. for GPU rendering).
pub trait ToFloat<Target> {
    fn to_float(&self) -> Target;
}

/// Quantized conversion back into exact fixed-point spatial representation.
pub trait FromFloat<Source> {
    fn from_float(val: Source) -> Self;
}

// --- Scalar Implementations ---

impl ToFloat<ScalarF> for Scalar {
    #[inline]
    fn to_float(&self) -> ScalarF {
        self.to_num::<ScalarF>()
    }
}

impl FromFloat<ScalarF> for Scalar {
    #[inline]
    fn from_float(val: ScalarF) -> Self {
        Scalar::from_num(val)
    }
}

// --- Angle Implementations ---

impl ToFloat<AngleF> for Angle {
    #[inline]
    fn to_float(&self) -> AngleF {
        cgmath::Rad(self.0.to_float())
    }
}

impl FromFloat<AngleF> for Angle {
    #[inline]
    fn from_float(val: AngleF) -> Self {
        cgmath::Rad(Scalar::from_num(val.0))
    }
}

// --- Vector Implementations ---

macro_rules! impl_vec_convert {
    ($fixed_type:ident, $float_type:ident, $($elem:ident),+ $(,)?) => {
        // Fixed -> Float Vector
        impl ToFloat<$float_type> for $fixed_type {
            #[inline]
            fn to_float(&self) -> $float_type {
                $float_type::new(
                    $( self.$elem.to_float() ),+
                )
            }
        }

        // Float -> Fixed Vector
        impl FromFloat<$float_type> for $fixed_type {
            #[inline]
            fn from_float(val: $float_type) -> Self {
                $fixed_type::new(
                    $( crate::Scalar::from_float(val.$elem) ),+
                )
            }
        }
    };
}

// Vec2
impl_vec_convert!(Vec2, Vec2F, x, y);

// Vec3
impl_vec_convert!(Vec3, Vec3F, x, y, z);

// Vec4
impl_vec_convert!(Vec4, Vec4F, x, y, z, w);

// --- Matrix4 Implementations ---

macro_rules! impl_matrix_convert {
    ($fixed_type:ident, $float_type:ident, $($col:ident . $elem:ident),+ $(,)?) => {
        // Fixed -> Float Matrix
                impl ToFloat<$float_type> for $fixed_type {
                    #[inline]
                    fn to_float(&self) -> $float_type {
                        $float_type::new(
                            $( self.$col.$elem.to_float() ),+
                        )
                    }
                }

                // Float -> Fixed Matrix
                impl FromFloat<$float_type> for $fixed_type {
                    #[inline]
                    fn from_float(val: $float_type) -> Self {
                        $fixed_type::new(
                            $( crate::Scalar::from_float(val.$col.$elem) ),+
                        )
                    }
                }
    };
}

// 2x2
impl_matrix_convert!(Mat2, Mat2F, x.x, x.y, y.x, y.y,);

// 3x3
impl_matrix_convert!(Mat3, Mat3F, x.x, x.y, x.z, y.x, y.y, y.z, z.x, z.y, z.z,);

// 4x4
impl_matrix_convert!(
    Mat4, Mat4F, x.x, x.y, x.z, x.w, y.x, y.y, y.z, y.w, z.x, z.y, z.z, z.w, w.x, w.y, w.z, w.w,
);
