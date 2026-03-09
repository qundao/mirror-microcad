// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, syntax::*};

impl Eval for std::rc::Rc<SourceFile> {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.scope(
            StackFrame::Source(self.id(), SymbolMap::default()),
            |context| self.statements.eval(context),
        )
    }
}
