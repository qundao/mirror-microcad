// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language types and values.

pub mod ty;
pub mod value;

mod color;

pub use color::Color;

pub use ty::{MatrixType, QuantityType, Ty, Type, TypeError, TypeResult, Unit};
pub use value::{Array, Quantity, Tuple, Value, ValueError, ValueList, ValueResult};

pub use microcad_lang_base::element::{BinaryOperator, UnaryOperator};

pub type Integer = fixed::FixedI64<fixed::types::extra::U0>;

pub type Scalar = fixed::FixedI128<fixed::types::extra::U32>;

/// 2D vector type.
pub type Vec2 = cgmath::Vector2<Scalar>;
/// 3D vector type.
pub type Vec3 = cgmath::Vector3<Scalar>;
/// 4D vector type.
pub type Vec4 = cgmath::Vector4<Scalar>;
/// 2D matrix type.
pub type Mat2 = cgmath::Matrix2<Scalar>;
/// 3D matrix type.
pub type Mat3 = cgmath::Matrix3<Scalar>;
/// 4D matrix type.
pub type Mat4 = cgmath::Matrix4<Scalar>;
/// Primitive angle type in radians.
pub type Angle = cgmath::Rad<Scalar>;

/// A length in mm
pub struct Length(pub Scalar);

/// A trait to implement binary and operators for [`Type`] and [`Value`].
pub trait Operators: Sized {
    type Err;

    /// Perform a binary operation.
    fn binary_op(self, op: BinaryOperator, rhs: Self) -> Result<Self, Self::Err>;

    /// Perform a unary operation.
    fn unary_op(self, op: UnaryOperator) -> Result<Self, Self::Err>;
}
