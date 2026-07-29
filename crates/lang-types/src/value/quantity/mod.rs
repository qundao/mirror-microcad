// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Matrix value type

pub mod ops;

use crate::ty::*;

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::{Integer, Length, Scalar};

const OUTPUT_PRECISION: i32 = 14;

/// A numeric value
#[derive(Clone, Debug, Display, Serialize, Deserialize)]
#[display(
    "{:.PRECISION$}{unit}",
    unit.denormalize(*value).to_num::<f64>(),
    PRECISION = OUTPUT_PRECISION as usize
)]
pub struct Quantity {
    /// The numeric value of the quantity.
    pub value: Scalar,
    /// The quantity type with a base unit.
    pub quantity_type: QuantityType,
    /// The original unit of the quantity,
    pub unit: Unit,
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        // 1. Ensure the types match first
        if self.quantity_type != other.quantity_type {
            return false;
        }

        // 2. Compare values within the allowed precision
        let epsilon = 10.0_f64.powi(-OUTPUT_PRECISION);
        (self.value - other.value).abs() < epsilon
    }
}

impl Quantity {
    /// Create a new quantity.
    pub fn new(value: Scalar, quantity_type: QuantityType) -> Self {
        Self {
            value,
            unit: quantity_type.base_unit(),
            quantity_type,
        }
    }
    /// Transforms the internal value using a closure.
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(Scalar) -> Scalar,
    {
        Self {
            value: f(self.value),
            quantity_type: self.quantity_type,
            unit: self.unit,
        }
    }

    /// Calculate the power of quantity.
    pub fn pow(&self, _rhs: &Quantity) -> Self {
        todo!()
    }

    /// Calculate the power of quantity and an integer.
    pub fn pow_int(&self, _rhs: &Integer) -> Self {
        todo!()
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.quantity_type == other.quantity_type {
            self.value.partial_cmp(&other.value)
        } else {
            None
        }
    }
}

impl From<Scalar> for Quantity {
    fn from(value: Scalar) -> Self {
        Self::new(value, QuantityType::Scalar)
    }
}

impl From<Integer> for Quantity {
    fn from(value: Integer) -> Self {
        Self::new(Scalar::from(value), QuantityType::Scalar)
    }
}

impl From<Length> for Quantity {
    fn from(length: Length) -> Self {
        Self::new(length.0, QuantityType::Length)
    }
}

impl Ty for Quantity {
    fn ty(&self) -> Type {
        Type::Quantity(self.quantity_type.clone())
    }
}

impl std::hash::Hash for Quantity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        bytemuck::bytes_of(&self.value).hash(state);
        self.quantity_type.hash(state)
    }
}
