// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

/// Evaluate the body into a value.
impl Eval for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            self.statements.eval(context)
        })
    }
}
