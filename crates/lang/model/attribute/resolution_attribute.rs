// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolution attribute.

use microcad_core::Scalar;

use crate::{ty::QuantityType, value::*};

/// Render resolution when rendering things e.g. to polygons or meshes.
#[derive(Debug, Clone)]
pub enum ResolutionAttribute {
    /// Linear resolution in millimeters (Default = 0.1mm)
    Absolute(Scalar),

    /// Relative resolution.
    Relative(Scalar),
}

impl Default for ResolutionAttribute {
    fn default() -> Self {
        Self::Absolute(0.1)
    }
}

impl From<ResolutionAttribute> for Value {
    fn from(resolution_attribute: ResolutionAttribute) -> Self {
        match resolution_attribute {
            ResolutionAttribute::Absolute(linear) => {
                Self::Quantity(Quantity::new(linear, QuantityType::Length))
            }
            ResolutionAttribute::Relative(relative) => {
                Self::Quantity(Quantity::new(relative, QuantityType::Scalar))
            }
        }
    }
}

impl TryFrom<Value> for ResolutionAttribute {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Scalar,
            }) => Ok(ResolutionAttribute::Relative(value)),
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Length,
            }) => Ok(ResolutionAttribute::Absolute(value)),
            _ => Err(ValueError::CannotConvert(
                value.to_string(),
                "ResolutionAttribute".to_string(),
            )),
        }
    }
}

impl std::fmt::Display for ResolutionAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolutionAttribute::Absolute(linear) => write!(f, "Linear({linear} mm)"),
            ResolutionAttribute::Relative(relative) => write!(f, "Relative({relative}%)"),
        }
    }
}
