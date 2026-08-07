// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language types and values.

use microcad_lang_base::element::{BinaryOperator, UnaryOperator};

use crate::{
    Array, Operators, Quantity, QuantityType, Scalar, Ty, Type, Unit, Value, ValueError,
    ValueResult,
};

impl Operators for Value {
    type Err = ValueError;

    /// Binary operation
    fn binary_op(self, op: BinaryOperator, rhs: Self) -> ValueResult {
        use BinaryOperator::*;
        let lhs = self;
        match op {
            Add => lhs + rhs,
            Subtract => lhs - rhs,
            Multiply => lhs * rhs,
            Divide => lhs / rhs,
            Union | Or => lhs | rhs,
            Intersect | And => lhs & rhs,
            PowerXor | Xor => lhs.pow(&rhs),
            GreaterThan => Ok((lhs > rhs).into()),
            LessThan => Ok((lhs < rhs).into()),
            GreaterEqual => Ok((lhs >= rhs).into()),
            LessEqual => Ok((lhs <= rhs).into()),
            Equal => Ok((lhs == rhs).into()),
            NotEqual => Ok((lhs != rhs).into()),
            _ => Err(ValueError::InvalidOperator(op.to_string())),
        }
    }

    /// Unary operation.
    fn unary_op(self, op: UnaryOperator) -> Result<Value, ValueError> {
        match op {
            UnaryOperator::Minus => -self,
            UnaryOperator::Not => !self,
            UnaryOperator::Plus => Ok(self),
        }
    }
}

impl std::ops::Neg for Value {
    type Output = ValueResult;

    fn neg(self) -> Self::Output {
        match self {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Quantity(q) => Ok(Value::Quantity(q.neg())),
            Value::Array(a) => -a,
            Value::Tuple(t) => -t.as_ref().clone(),
            _ => Err(ValueError::InvalidOperator("-".into())),
        }
    }
}

impl std::ops::Not for Value {
    type Output = ValueResult;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Array(a) => !a,
            Value::Tuple(t) => !t.as_ref().clone(),
            _ => Err(ValueError::InvalidOperator("!".into())),
        }
    }
}

/// Rules for operator `+`.
impl std::ops::Add for Value {
    type Output = ValueResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Add two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs + rhs)),
            // Add a quantity to an integer
            (Value::Integer(lhs), Value::Quantity(rhs)) => lhs + rhs,
            // Add an integer to a quantity
            (Value::Quantity(lhs), Value::Integer(rhs)) => lhs + rhs,
            // Add two scalars
            (Value::Quantity(lhs), Value::Quantity(rhs)) => lhs + rhs,
            // Concatenate two strings
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::String(lhs + &rhs)),
            // Concatenate two lists
            (Value::Array(lhs), Value::Array(rhs)) => {
                if lhs.ty() != rhs.ty() {
                    return Err(ValueError::CannotCombineVecOfDifferentType(
                        lhs.ty(),
                        rhs.ty(),
                    ));
                }

                Ok(Value::Array(Array::from_values(
                    lhs.iter().chain(rhs.iter()).cloned().collect(),
                )))
            }
            // Add a value to an array.
            (Value::Array(lhs), rhs) => Ok((lhs + rhs)?),
            // Add two tuples of the same type: (x = 1., y = 2.) + (x = 3., y = 4.)
            (Value::Tuple(lhs), Value::Tuple(rhs)) => Ok((*lhs + *rhs)?.into()),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} + {rhs}"))),
        }
    }
}

/// Rules for operator `-`.
impl std::ops::Sub for Value {
    type Output = ValueResult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Subtract two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs - rhs)),
            // Subtract an scalar and an integer
            (Value::Quantity(lhs), Value::Integer(rhs)) => lhs - rhs,
            // Subtract an integer and a scalar
            (Value::Integer(lhs), Value::Quantity(rhs)) => lhs - rhs,
            // Subtract two numbers
            (Value::Quantity(lhs), Value::Quantity(rhs)) => lhs - rhs,
            // Subtract value to an array: `[1,2,3] - 1 = [0,1,2]`.
            (Value::Array(lhs), rhs) => lhs - rhs,
            // Subtract two tuples of the same type: (x = 1., y = 2.) - (x = 3., y = 4.)
            (Value::Tuple(lhs), Value::Tuple(rhs)) => Ok((*lhs - *rhs)?.into()),

            // Boolean difference operator for models
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} - {rhs}"))),
        }
    }
}

/// Rules for operator `*`.
impl std::ops::Mul for Value {
    type Output = ValueResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Multiply two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs * rhs)),
            // Multiply an integer and a scalar, result is scalar
            (Value::Integer(lhs), Value::Quantity(rhs)) => lhs * rhs,
            // Multiply a scalar and an integer, result is scalar
            (Value::Quantity(lhs), Value::Integer(rhs)) => lhs * rhs,
            // Multiply two scalars
            (Value::Quantity(lhs), Value::Quantity(rhs)) => lhs * rhs,
            (Value::Array(array), value) | (value, Value::Array(array)) => Ok((array * value)?),

            (Value::Tuple(tuple), value) | (value, Value::Tuple(tuple)) => {
                Ok((tuple.as_ref().clone() * value)?.into())
            }
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} * {rhs}"))),
        }
    }
}

/// Multiply a Unit with a value. Used for unit bundling: `[1,2,3]mm`.
///
/// `[1,2,3]mm` is a shortcut for `[1,2,3] * 1mm`.
impl std::ops::Mul<Unit> for Value {
    type Output = ValueResult;

    fn mul(self, unit: Unit) -> Self::Output {
        match (self, unit.ty()) {
            (value, Type::Quantity(QuantityType::Scalar)) | (value, Type::Integer) => Ok(value),
            (Value::Integer(i), Type::Quantity(quantity_type)) => Ok(Value::Quantity(
                Quantity::new(unit.normalize(Scalar::from(i)), quantity_type),
            )),
            (Value::Quantity(quantity), Type::Quantity(quantity_type)) => {
                quantity * Quantity::new(unit.factor(), quantity_type)
            }
            (Value::Array(array), Type::Quantity(quantity_type)) => {
                Ok((array * Value::Quantity(Quantity::new(unit.factor(), quantity_type)))?)
            }
            (value, _) => Err(ValueError::CannotAddUnitToValueWithUnit(value.to_string())),
        }
    }
}

/// Rules for operator `/`.
impl std::ops::Div for Value {
    type Output = ValueResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Division with scalar result
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Quantity(
                (Scalar::from(lhs) / Scalar::from(rhs)).into(),
            )),
            (Value::Quantity(lhs), Value::Integer(rhs)) => lhs / rhs,
            (Value::Integer(lhs), Value::Quantity(rhs)) => lhs / rhs,
            (Value::Quantity(lhs), Value::Quantity(rhs)) => lhs / rhs,
            (Value::Array(array), value) => Ok((array / value)?),
            (Value::Tuple(tuple), value) => Ok((tuple.as_ref().clone() / value)?.into()),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} / {rhs}"))),
        }
    }
}

/// Rules for operator `|`` (union).
impl std::ops::BitOr for Value {
    type Output = ValueResult;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(lhs | rhs)),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} | {rhs}"))),
        }
    }
}

/// Rules for operator `&` (intersection).
impl std::ops::BitAnd for Value {
    type Output = ValueResult;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(lhs & rhs)),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} & {rhs}"))),
        }
    }
}
