// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad quantity type

use strum::IntoStaticStr;

use crate::lower::ir;

/// A quantity type with
#[derive(Clone, Debug, IntoStaticStr, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuantityType {
    /// A unitless scalar value.
    Scalar,
    /// Length in mm.
    Length,
    /// Area in mm².
    Area,
    /// Volume in mm³.
    Volume,
    /// Density in g/mm³
    Density,
    /// An angle in radians.
    Angle,
    /// Weight of a specific volume of material.
    Weight,
    /// An invalid, unsupported quantity type.
    Invalid,
}

impl QuantityType {
    /// Return base unit
    pub fn base_unit(&self) -> ir::Unit {
        match self {
            QuantityType::Scalar => ir::Unit::None,
            QuantityType::Length => ir::Unit::Millimeter,
            QuantityType::Area => ir::Unit::Millimeter2,
            QuantityType::Volume => ir::Unit::Millimeter3,
            QuantityType::Density => ir::Unit::GramPerMeter3,
            QuantityType::Angle => ir::Unit::Rad,
            QuantityType::Weight => ir::Unit::Gram,
            QuantityType::Invalid => todo!(),
        }
    }
}

impl std::fmt::Display for QuantityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")
    }
}

impl std::ops::Mul for QuantityType {
    type Output = QuantityType;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == Self::Invalid || rhs == Self::Invalid {
            return Self::Invalid;
        }
        if self == QuantityType::Scalar {
            return rhs;
        }
        if rhs == QuantityType::Scalar {
            return self;
        }

        match (self, rhs) {
            (QuantityType::Length, QuantityType::Length) => QuantityType::Area,
            (QuantityType::Length, QuantityType::Area)
            | (QuantityType::Area, QuantityType::Length) => QuantityType::Volume,
            (_, _) => QuantityType::Invalid,
        }
    }
}

impl std::ops::Div for QuantityType {
    type Output = QuantityType;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == self {
            return QuantityType::Scalar;
        }
        if rhs == QuantityType::Scalar {
            return self;
        }

        match (self, rhs) {
            (QuantityType::Area, QuantityType::Length)
            | (QuantityType::Volume, QuantityType::Area) => QuantityType::Length,
            (QuantityType::Volume, QuantityType::Length) => QuantityType::Area,
            (_, _) => QuantityType::Invalid,
        }
    }
}
