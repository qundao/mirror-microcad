// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval<Value> for ReturnStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        self.grant(context)?;
        log::debug!("Evaluating return statement to value: {self}");
        if let Some(result) = &self.result {
            let result = result.eval(context)?;
            log::debug!("returning {result}");
            Ok(Value::Return(Box::new(result)))
        } else {
            Ok(Value::Return(Value::None.into()))
        }
    }
}
