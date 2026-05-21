// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `std::ops` impls for Model

use microcad_core::{BooleanOp, Integer};
use microcad_lang_base::SrcRef;

use crate::{
    eval::EvalResult,
    model::{Model, Models},
};

pub type ModelResult = EvalResult<Model>;

impl std::ops::Sub for Model {
    type Output = ModelResult;

    fn sub(self, rhs: Self) -> Self::Output {
        Ok(self.boolean_op(BooleanOp::Subtract, rhs))
    }
}

/// Mul: `1 * Circle(r)`;
impl std::ops::Mul<Model> for Integer {
    type Output = ModelResult;

    fn mul(self, rhs: Model) -> Self::Output {
        Ok(Models::from(rhs.multiply(self)).to_multiplicity(SrcRef::none()))
    }
}

/// union operator `|`
impl std::ops::BitOr for Model {
    type Output = ModelResult;

    fn bitor(self, rhs: Self) -> Self::Output {
        Ok(self.boolean_op(BooleanOp::Union, rhs))
    }
}

/// intersection operator `&`
impl std::ops::BitAnd for Model {
    type Output = ModelResult;

    fn bitand(self, rhs: Self) -> Self::Output {
        Ok(self.boolean_op(BooleanOp::Intersect, rhs))
    }
}
