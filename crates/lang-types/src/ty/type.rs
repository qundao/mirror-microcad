// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::{model::ModelOutputType, ty::*};

/// µcad Basic Types
#[derive(Clone, Debug, Default, PartialEq, From, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Any type allowed.
    #[default]
    Any,
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
    /// A list of elements, with a optional: `[Scalar]`.
    /// `List` is a type alias for `[Any]`.
    List(Box<Type>),
    /// A named tuple of elements: `(x: Scalar, y: String)`.
    Tuple(Box<TupleType>),
    /// Matrix type: `Matrix3x3`.
    Matrix(MatrixType),
    /// Function type: (x: Length, y: Length) -> Length,
    Function(FunctionType),
    /// Model.
    Model(ModelOutputType),
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

    /// Shortcut to create an angle type.
    pub fn angle() -> Self {
        Self::Quantity(QuantityType::Angle)
    }

    /// Short-cut for a `Color` type
    pub fn color() -> Self {
        TupleType::color().into()
    }

    /// Short-cut for a `List` type, or `[Any]`, respectively.
    pub fn list() -> Self {
        Self::List(Box::new(Type::Any))
    }

    /// Short-cut for a matrix type.
    pub fn matrix(rows: usize, columns: usize) -> Self {
        Self::Matrix(MatrixType::new(rows, columns))
    }

    /// Check if the type is an list of the given type `ty`
    pub fn is_list_of(&self, ty: &Type) -> bool {
        match self {
            Self::List(list_type) => list_type.as_ref() == ty,
            _ => false,
        }
    }

    /// Check if types are compatible.
    pub fn is_compatible_to(&self, rhs: &Self) -> bool {
        rhs == self
            || (*self == Type::Integer && *rhs == Type::scalar())
            || (*rhs == Type::Integer && *self == Type::scalar())
    }

    /// Returns if the given type or it's inner type matches the given type.
    pub fn matches(&self, other: &Type) -> bool {
        match (self, other) {
            (lhs, Type::Quantity(QuantityType::Scalar)) if *lhs != Type::Any => {
                self == &Type::scalar() || self == &Type::Integer
            }
            // Invalid types never match
            (Type::Tuple(ty_s), Type::Tuple(ty_p)) => ty_s.matches_multiplicity(ty_p),
            (Type::Invalid, _) | (_, Type::Invalid) => false,
            (Type::Any, _) | (_, Type::Any) => true,
            _ => self == other,
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
            "Color" => Ok(Type::color()),
            "Vec2" => Ok(Type::Tuple(Box::new(TupleType::vec2()))),
            "Vec3" => Ok(Type::Tuple(Box::new(TupleType::vec3()))),
            "Mat3" => Ok(Type::matrix(3, 3)),
            "Size2" => Ok(Type::Tuple(Box::new(TupleType::size2()))),
            "Any" => Ok(Type::Any),
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
            "Model" => Ok(Type::Model(ModelOutputType::Any)),
            _ => Err(TypeError::UnknownType(ty.to_string())),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "<NO TYPE>"),
            Self::Any => write!(f, "Any"),
            Self::Integer => write!(f, "Integer"),
            Self::Quantity(quantity) => write!(f, "{quantity}"),
            Self::String => write!(f, "String"),
            Self::Bool => write!(f, "Bool"),
            Self::List(t) => write!(f, "[{t}]"),
            Self::Tuple(t) => write!(f, "{t}"),
            Self::Matrix(t) => write!(f, "{t}"),
            Self::Function(t) => write!(f, "{t}"),
            Self::Model(output_type) => write!(f, "Model({output_type})"),
        }
    }
}

impl From<TupleType> for Type {
    fn from(ty: TupleType) -> Self {
        Self::Tuple(Box::new(ty))
    }
}
