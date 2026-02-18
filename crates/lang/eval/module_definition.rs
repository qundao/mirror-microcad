// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for ModuleDefinition {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        self.grant(context)?;
        context.scope(
            StackFrame::Module(self.id.clone(), Default::default()),
            |context| {
                if let Some(body) = &self.body {
                    body.statements.eval(context)
                } else {
                    Ok(Value::None)
                }
            },
        )
    }
}
