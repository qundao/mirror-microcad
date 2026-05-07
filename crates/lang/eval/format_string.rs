// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, lower::ir};

impl Eval for ir::FormatExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let value: Value = self.expression.eval(context)?;
        Ok(Value::String(format!("{value}")))
    }
}

impl Eval for ir::FormatString {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let mut result = String::new();
        for elem in &*self.0 {
            match elem {
                ir::FormatStringInner::String(s) => result += &s.value,
                ir::FormatStringInner::FormatExpression(expr) => match expr.eval(context) {
                    Ok(Value::String(s)) => result += &s,
                    Err(e) => return Err(e),
                    _ => unreachable!("FormatExpression must always evaluate to a string"),
                },
            }
        }
        Ok(Value::String(result))
    }
}
