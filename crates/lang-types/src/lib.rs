// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language types and values.

pub mod ty;
pub mod value;

pub use ty::{MatrixType, QuantityType, Ty, Type, TypeError, Unit};
pub use value::{Array, Quantity, Tuple, Value, ValueError, ValueList, ValueResult};

pub use microcad_lang_base::element::{BinaryOperator, UnaryOperator};

pub use microcad_core::{
    Angle, Color, Integer, Length, Mat2, Mat3, Mat4, Scalar, Size2, Vec2, Vec3, Vec4,
}; // TODO Move these typedefs into this crate.

pub trait Operators: Sized {
    type Err;

    fn binary_op(self, op: BinaryOperator, rhs: Self) -> Result<Self, Self::Err>;

    fn unary_op(self, op: UnaryOperator) -> Result<Self, Self::Err>;
}
