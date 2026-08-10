// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Math functions for Value

use crate::{MathOps, Quantity, Ty, Value, ValueError, ValueResult};

impl MathOps for Value {
    type Error = ValueError;
    type Output = Value;

    /// Calculate the absolute value.
    fn abs(&self) -> ValueResult {
        match self {
            Value::Integer(i) => Ok(Value::Integer(i.abs())),
            Value::Quantity(q) => q.abs(),
            val => Err(ValueError::InvalidMathOperand(val.ty())),
        }
    }

    /// Calculate the square root.
    fn sqrt(&self) -> ValueResult {
        match self {
            Value::Integer(i) => Quantity::from(*i).sqrt(),
            Value::Quantity(q) => q.sqrt(),
            val => Err(ValueError::InvalidMathOperand(val.ty())),
        }
    }

    /// Calculate the sine (expects radians or Quantity with angle units).
    fn sin(&self) -> ValueResult {
        match self {
            Value::Integer(i) => Quantity::from(*i).sin(),
            Value::Quantity(q) => q.sin(),
            val => Err(ValueError::InvalidMathOperand(val.ty())),
        }
    }

    /// Calculate the cosine (expects radians or Quantity with angle units).
    fn cos(&self) -> ValueResult {
        match self {
            Value::Integer(i) => Quantity::from(*i).cos(),
            Value::Quantity(q) => q.cos(),
            val => Err(ValueError::InvalidMathOperand(val.ty())),
        }
    }

    /// Calculate the tangent (expects radians or Quantity with angle units).
    fn tan(&self) -> ValueResult {
        match self {
            Value::Integer(i) => Quantity::from(*i).tan(),
            Value::Quantity(q) => q.tan(),
            val => Err(ValueError::InvalidMathOperand(val.ty())),
        }
    }
}
