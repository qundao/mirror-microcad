// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language types and values.

pub mod math;
pub mod model;
pub mod ty;
pub mod value;

mod color;
pub use color::Color;

mod arguments;
pub use arguments::{ArgumentValue, ArgumentValueList, Arguments};

use derive_more::{Deref, DerefMut, Display};
use serde::{Deserialize, Serialize};
pub use ty::{
    CallSignature, FunctionType, MatrixType, QuantityType, TupleType, Ty, Type, TypeError,
    TypeResult, Unit,
};
pub use value::{List, Quantity, Tuple, Value, ValueAccess, ValueError, ValueList, ValueResult};

pub use model::{
    Model, ModelOutputType, ModelTree, NodeMut as ModelNodeMut, NodeRef as ModelNodeRef,
};

pub use math::MathOps;

pub use microcad_lang_base::{
    Identifier,
    element::{BinaryOperator, UnaryOperator},
};

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

pub type ScalarF = f64;

pub type AngleF = cgmath::Rad<ScalarF>;

/// 3D vector type.
pub type Vec3F = cgmath::Vector3<ScalarF>;

/// Matrix 3x3 floating point type.
pub type Mat3F = cgmath::Matrix3<ScalarF>;

/// A length in mm
#[derive(
    Clone,
    Debug,
    Display,
    Copy,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
#[display("{_0}mm")]
pub struct Length(pub Scalar);

impl Length {
    /// Return a new length from millimeters.
    pub fn mm(mm: f64) -> Self {
        Self(Scalar::from_num(mm))
    }
}

/// A trait to implement binary and operators for [`Type`] and [`Value`].
pub trait Operators: Sized {
    type Err;

    /// Perform a binary operation.
    fn binary_op(self, op: BinaryOperator, rhs: Self) -> Result<Self, Self::Err>;

    /// Perform a unary operation.
    fn unary_op(self, op: UnaryOperator) -> Result<Self, Self::Err>;
}
