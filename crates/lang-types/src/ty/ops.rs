// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Type;

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
            (lhs, rhs) => unimplemented!("Division for {lhs} / {rhs}"),
        }
    }
}
