// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Math functions for Quantity

use crate::{MathOps, Quantity, Scalar, Value, ValueError, ValueResult};

impl MathOps for Quantity {
    type Output = Value;
    type Error = ValueError;

    fn sqrt(&self) -> ValueResult {
        if self.value < 0 {
            Err(ValueError::DomainError(
                "Cannot compute sqrt of negative number".into(),
            ))
        } else {
            Ok(self.clone().map(|v| v.sqrt()).into())
        }
    }

    fn abs(&self) -> ValueResult {
        Ok(self.clone().map(|v| v.abs()).into())
    }

    fn sin(&self) -> ValueResult {
        Ok(self
            .clone()
            .map(|v| Scalar::from_num(v.to_num::<f64>().sin()))
            .into())
    }

    fn cos(&self) -> Result<Self::Output, Self::Error> {
        Ok(self
            .clone()
            .map(|v| Scalar::from_num(v.to_num::<f64>().cos()))
            .into())
    }

    fn tan(&self) -> Result<Self::Output, Self::Error> {
        Ok(self
            .clone()
            .map(|v| Scalar::from_num(v.to_num::<f64>().tan()))
            .into())
    }
}
