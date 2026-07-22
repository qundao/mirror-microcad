// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation entities.
//!
//! Every evaluation of any *symbol* leads to a [`Value`] which then might continued
//! to process or ends up as the overall evaluation result.

mod array;
mod matrix;
pub mod ops;
mod quantity;
mod tuple;
mod value_access;
mod value_error;
mod value_list;

pub use array::*;
pub use matrix::*;
pub use quantity::*;
pub use tuple::*;
pub use value_access::*;
pub use value_error::*;
pub use value_list::*;

use crate::{Angle, Color, Integer, Length, Mat3, Scalar, Size2, Vec2, Vec3, ty::*};

use derive_more::From;
use microcad_lang_base::SrcRef;
use serde::{Deserialize, Serialize};

pub type ValueResult<Type = Value> = std::result::Result<Type, ValueError>;

/// A variant value with attached source code reference.
#[derive(Clone, Debug, Default, PartialEq, From, Serialize, Deserialize)]
pub enum Value {
    /// Invalid value (used for error handling).
    #[default]
    None,
    /// A quantity value.
    Quantity(Quantity),
    /// A boolean value.
    Bool(bool),
    /// An integer value.
    Integer(Integer),
    /// A string value.
    String(String),
    /// A list of values with a common type.
    Array(Array),
    /// A tuple of named items.
    Tuple(Box<Tuple>),
    /// A matrix.
    Matrix(Box<Matrix>),
}

impl Value {
    /// Check if the value is invalid.
    pub fn is_invalid(&self) -> bool {
        matches!(self, Value::None)
    }

    /// Calculate the power of two values, if possible.
    pub fn pow(&self, rhs: &Value) -> ValueResult {
        match (&self, rhs) {
            (Value::Quantity(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity(lhs.pow(rhs))),
            (Value::Quantity(lhs), Value::Integer(rhs)) => Ok(Value::Quantity(lhs.pow_int(rhs))),
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs.pow(*rhs as u32))),
            _ => Err(ValueError::InvalidOperator("^".to_string())),
        }
    }

    /// Try to convert to [`String`].
    pub fn try_string(&self) -> Result<String, ValueError> {
        match self {
            Value::String(s) => return Ok(s.clone()),
            Value::Integer(i) => return Ok(i.to_string()),
            _ => {}
        }

        Err(ValueError::CannotConvert(self.to_string(), "String".into()))
    }

    /// Try to convert to [`Scalar`].
    pub fn try_scalar(&self) -> Result<Scalar, ValueError> {
        match self {
            Value::Quantity(q) => return Ok(q.value),
            Value::Integer(i) => return Ok((*i) as f64),
            _ => {}
        }

        Err(ValueError::CannotConvert(self.to_string(), "Scalar".into()))
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // integer type
            (Value::Integer(lhs), Value::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Value::Quantity(lhs), Value::Quantity(rhs)) => lhs.partial_cmp(rhs),
            (
                Value::Quantity(Quantity {
                    value,
                    quantity_type: QuantityType::Scalar,
                    ..
                }),
                Value::Integer(rhs),
            ) => value.partial_cmp(&(*rhs as Scalar)),
            _ => {
                log::warn!("unhandled type mismatch between {self} and {other}");
                None
            }
        }
    }
}

impl crate::ty::Ty for Value {
    fn ty(&self) -> Type {
        match self {
            Value::None => Type::Invalid,
            Value::Integer(_) => Type::Integer,
            Value::Quantity(q) => q.ty(),
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Array(list) => list.ty(),
            Value::Tuple(tuple) => tuple.ty(),
            Value::Matrix(matrix) => matrix.ty(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::None => write!(f, "<NO VALUE>"),
            Value::Integer(n) => write!(f, "{n}"),
            Value::Quantity(q) => write!(f, "{q}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Array(l) => write!(f, "{l}"),
            Value::Tuple(t) => write!(f, "{t}"),
            Value::Matrix(m) => write!(f, "{m}"),
        }
    }
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::None => std::mem::discriminant(&Value::None).hash(state),
            Value::Quantity(quantity) => quantity.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::Integer(i) => i.hash(state),
            Value::String(s) => s.hash(state),
            Value::Array(array) => array.hash(state),
            Value::Tuple(tuple) => tuple.hash(state),
            Value::Matrix(matrix) => matrix.hash(state),
        }
    }
}

macro_rules! impl_try_from {
    ($($variant:ident),+ => $ty:ty ) => {
        impl TryFrom<Value> for $ty {
            type Error = ValueError;

            fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
                match value {
                    $(Value::$variant(v) => Ok(v),)*
                    value => Err(ValueError::CannotConvert(value.to_string(), stringify!($ty).into())),
                }
            }
        }

        impl TryFrom<&Value> for $ty {
            type Error = ValueError;

            fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
                match value {
                    $(Value::$variant(v) => Ok(v.clone().into()),)*
                    value => Err(ValueError::CannotConvert(value.to_string(), stringify!($ty).into())),
                }
            }
        }
    };
}

impl_try_from!(Integer => i64);
impl_try_from!(Bool => bool);
impl_try_from!(String => String);

impl TryFrom<&Value> for Scalar {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(*i as Scalar),
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Scalar,
                ..
            }) => Ok(*value),
            _ => Err(ValueError::CannotConvert(
                value.to_string(),
                "Scalar".into(),
            )),
        }
    }
}

impl TryFrom<Value> for Scalar {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(i as Scalar),
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Scalar,
                ..
            }) => Ok(value),
            _ => Err(ValueError::CannotConvert(
                value.to_string(),
                "Scalar".into(),
            )),
        }
    }
}

impl TryFrom<&Value> for Angle {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Angle,
                ..
            }) => Ok(cgmath::Rad(*value)),
            _ => Err(ValueError::CannotConvert(value.to_string(), "Angle".into())),
        }
    }
}

impl TryFrom<&Value> for Length {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Length,
                ..
            }) => Ok(Length(*value)),
            _ => Err(ValueError::CannotConvert(
                value.to_string(),
                "Length".into(),
            )),
        }
    }
}

impl TryFrom<&Value> for Size2 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tuple(tuple) => Ok(tuple.as_ref().try_into()?),
            _ => Err(ValueError::CannotConvert(value.to_string(), "Size2".into())),
        }
    }
}

impl TryFrom<&Value> for Mat3 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Value::Matrix(m) = value
            && let Matrix::Matrix3(matrix3) = m.as_ref()
        {
            return Ok(*matrix3);
        }

        Err(ValueError::CannotConvert(
            value.to_string(),
            "Matrix3".into(),
        ))
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::Integer(value as Integer)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Quantity((f as Scalar).into())
    }
}

impl From<Scalar> for Value {
    fn from(scalar: Scalar) -> Self {
        Value::Quantity(scalar.into())
    }
}

impl From<Length> for Value {
    fn from(length: Length) -> Self {
        Value::Quantity(length.into())
    }
}

impl From<Size2> for Value {
    fn from(value: Size2) -> Self {
        Self::Tuple(Box::new(value.into()))
    }
}

impl From<Color> for Value {
    fn from(color: Color) -> Self {
        Self::Tuple(Box::new(color.into()))
    }
}

impl From<Vec3> for Value {
    fn from(v: Vec3) -> Self {
        Self::Tuple(Box::new(v.into()))
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self::Array(iter.into_iter().collect())
    }
}
