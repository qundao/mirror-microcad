// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type

use serde::Serialize;

use crate::ty::*;

/// µcad Basic Types
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum Type {
    /// Invalid type (used for error handling)
    Invalid,
    /// A 64-bit integer number: `Integer: 10`.
    Integer,
    /// A quantity type: `Scalar: 1.0`, `Length: 1.0mm`
    Quantity(QuantityType),
    /// A string.
    String,
    /// A boolean: `true`, `false`.
    Bool,
    /// An array of elements of the same type: `[Scalar]`.
    Array(Box<Type>),
    /// A named tuple of elements: `(x: Scalar, y: String)`.
    Tuple(Box<TupleType>),
    /// Matrix type: `Matrix3x3`.
    Matrix(MatrixType),
    /// Model.
    Model,
}

impl Type {
    /// Shortcut to create a scalar type.
    pub fn scalar() -> Self {
        Self::Quantity(QuantityType::Scalar)
    }

    /// Shortcut to create a length type.
    pub fn length() -> Self {
        Self::Quantity(QuantityType::Length)
    }

    /// Check if the type is an array of the given type `ty`
    pub fn is_array_of(&self, ty: &Type) -> bool {
        match self {
            Self::Array(array_type) => array_type.as_ref() == ty,
            _ => false,
        }
    }

    /// Check if types are compatible.
    pub fn is_compatible_to(&self, rhs: &Self) -> bool {
        rhs == self
            || (*self == Type::Integer && *rhs == Type::scalar())
            || (*rhs == Type::Integer && *self == Type::scalar())
    }

    /// Returns if the given type or it's inner type matches the given parameter type.
    pub fn is_matching(&self, param_type: &Type) -> bool {
        match (self, param_type) {
            (_, Type::Quantity(QuantityType::Scalar)) => {
                self == &Type::scalar()
                    || self == &Type::Integer
                    || self.is_array_of(&Type::scalar())
                    || self.is_array_of(&Type::Integer)
            }
            (Type::Tuple(ty_s), Type::Tuple(ty_p)) => ty_s.is_matching(ty_p),
            _ => self == param_type || self.is_array_of(param_type),
        }
    }
}

impl std::str::FromStr for Type {
    type Err = TypeError;

    /// Parse a type from &str
    fn from_str(ty: &str) -> Result<Self, TypeError> {
        if let Some(dimensions) = ty.strip_prefix("Matrix") {
            let (x, y) = dimensions
                .split_once('x')
                .unwrap_or((dimensions, dimensions));
            let x = usize::from_str(x).map_err(|_| TypeError::InvalidMatrixType(ty.to_string()))?;
            let y = usize::from_str(y).map_err(|_| TypeError::InvalidMatrixType(ty.to_string()))?;
            return Ok(Type::Matrix(MatrixType::new(x, y)));
        }

        match ty {
            "Color" => Ok(Type::Tuple(Box::new(TupleType::new_color()))),
            "Vec2" => Ok(Type::Tuple(Box::new(TupleType::new_vec2()))),
            "Vec3" => Ok(Type::Tuple(Box::new(TupleType::new_vec3()))),
            "Size2" => Ok(Type::Tuple(Box::new(TupleType::new_size2()))),
            "Integer" => Ok(Type::Integer),
            "Bool" => Ok(Type::Bool),
            "String" => Ok(Type::String),
            "Scalar" => Ok(Type::Quantity(QuantityType::Scalar)),
            "Length" => Ok(Type::Quantity(QuantityType::Length)),
            "Area" => Ok(Type::Quantity(QuantityType::Area)),
            "Angle" => Ok(Type::Quantity(QuantityType::Angle)),
            "Volume" => Ok(Type::Quantity(QuantityType::Volume)),
            "Weight" => Ok(Type::Quantity(QuantityType::Weight)),
            "Density" => Ok(Type::Quantity(QuantityType::Density)),
            "Model" => Ok(Type::Model),
            _ => Err(TypeError::UnknownType(ty.to_string())),
        }
    }
}

impl std::ops::Mul for Type {
    type Output = Type;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == Self::Invalid || rhs == Self::Invalid {
            return Self::Invalid;
        }

        match (self, rhs) {
            (Type::Integer, ty) | (ty, Type::Integer) => ty,
            (Type::Quantity(lhs), Type::Quantity(rhs)) => Type::Quantity(lhs * rhs),
            (ty, Type::Array(array_type)) | (Type::Array(array_type), ty) => *array_type * ty,
            (Type::Tuple(_), _) | (_, Type::Tuple(_)) => todo!(),
            (Type::Matrix(_), _) | (_, Type::Matrix(_)) => todo!(),
            (lhs, rhs) => unimplemented!("Multiplication for {lhs} * {rhs}"),
        }
    }
}

impl std::ops::Div for Type {
    type Output = Type;

    fn div(self, rhs: Self) -> Self::Output {
        if self == Self::Invalid || rhs == Self::Invalid {
            return Self::Invalid;
        }

        match (self, rhs) {
            (Type::Integer, ty) | (ty, Type::Integer) => ty,
            (Type::Quantity(lhs), Type::Quantity(rhs)) => Type::Quantity(lhs / rhs),
            (Type::Array(array_type), ty) => *array_type / ty,
            (Type::Tuple(_), _) => todo!(),
            (Type::Matrix(_), _) | (_, Type::Matrix(_)) => todo!(),
            (lhs, rhs) => unimplemented!("Division for {lhs} * {rhs}"),
        }
    }
}

impl From<QuantityType> for Type {
    fn from(value: QuantityType) -> Self {
        Type::Quantity(value)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "<NO TYPE>"),
            Self::Integer => write!(f, "Integer"),
            Self::Quantity(quantity) => write!(f, "{quantity}"),
            Self::String => write!(f, "String"),
            Self::Bool => write!(f, "Bool"),
            Self::Array(t) => write!(f, "[{t}]"),
            Self::Tuple(t) => write!(f, "{t}"),
            Self::Matrix(t) => write!(f, "{t}"),
            Self::Model => write!(f, "Model"),
        }
    }
}
