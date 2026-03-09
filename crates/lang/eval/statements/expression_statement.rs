// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for ExpressionStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        log::debug!("Evaluating expression statementto value:\n{self}");

        let value: Value = self.expression.eval(context)?;
        match value {
            Value::Model(model) => {
                let attributes = self.attribute_list.eval(context)?;
                model
                    .borrow_mut()
                    .attributes
                    .append(&mut attributes.clone());
                Ok(Value::Model(model))
            }
            _ => Ok(value),
        }
    }
}
