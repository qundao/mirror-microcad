// Copyright © 2024-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type

use crate::ty::*;

/// µcad Basic Types
#[derive(Clone, PartialEq, Eq, Hash)]
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
    /// used for assert_valid() and assert_invalid()
    Target,
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
            (Type::Tuple(ty_s), Type::Tuple(ty_p)) => ty_s.matches(ty_p),
            _ => self == param_type || self.is_array_of(param_type),
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
            Self::Invalid => write!(f, crate::invalid_no_ansi!(TYPE)),
            Self::Integer => write!(f, "Integer"),
            Self::Quantity(quantity) => write!(f, "{quantity}"),
            Self::String => write!(f, "String"),
            Self::Bool => write!(f, "Bool"),
            Self::Array(t) => write!(f, "[{t}]"),
            Self::Tuple(t) => write!(f, "{t}"),
            Self::Matrix(t) => write!(f, "{t}"),
            Self::Model => write!(f, "Model"),
            Self::Target => write!(f, "Target"),
        }
    }
}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, crate::invalid!(TYPE)),
            Self::Integer => write!(f, "Integer"),
            Self::Quantity(quantity) => write!(f, "{quantity}"),
            Self::String => write!(f, "String"),
            Self::Bool => write!(f, "Bool"),
            Self::Array(t) => write!(f, "[{t}]"),
            Self::Tuple(t) => write!(f, "{t}"),
            Self::Matrix(t) => write!(f, "{t}"),
            Self::Model => write!(f, "Models"),
            Self::Target => write!(f, "Target"),
        }
    }
}

#[test]
fn type_matching() {
    assert!(Type::scalar().is_matching(&Type::scalar()));
    assert!(!Type::scalar().is_matching(&Type::Integer));
    assert!(Type::Integer.is_matching(&Type::scalar()));
    assert!(!Type::scalar().is_matching(&Type::String));
    assert!(!Type::String.is_matching(&Type::scalar()));
}
