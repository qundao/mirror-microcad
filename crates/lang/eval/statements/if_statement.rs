// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement evaluation.

use crate::{eval::*, syntax::*, value::*};

impl Eval for IfStatement {
    fn eval(&self, context: &mut crate::eval::EvalContext) -> crate::eval::EvalResult<Value> {
        log::debug!("Evaluating if statement to value: {self}");

        let cond = self.cond.eval(context)?;
        match cond {
            Value::Bool(true) => Ok(self.body.eval(context)?),
            Value::Bool(false) => {
                if let Some(next) = &self.next_if {
                    next.eval(context)
                } else if let Some(body) = &self.body_else {
                    Ok(body.eval(context)?)
                } else {
                    unreachable!("missing else (MISSED IN RESOLVE)")
                }
            }
            _ => {
                context.error(
                    self,
                    EvalError::IfConditionIsNotBool {
                        condition: cond.to_string(),
                        src_ref: self.cond.src_ref(),
                    },
                )?;
                Ok(Value::None)
            }
        }
    }
}
