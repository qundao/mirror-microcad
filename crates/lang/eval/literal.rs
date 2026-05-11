// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, lower::ir};

impl Eval for ir::NumberLiteral {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Value> {
        Ok(self.value())
    }
}

impl Eval for ir::Literal {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Value> {
        Ok(self.value().clone())
    }
}
