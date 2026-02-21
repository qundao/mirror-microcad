// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

/// Evaluate the body into a value.
impl Eval<Value> for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            self.statements.eval(context)
        })
    }
}

impl Eval<Option<Model>> for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<Model>> {
        self.eval(context).map(Some)
    }
}

impl Eval<Models> for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Models> {
        self.statements.eval(context)
    }
}

impl Eval<Model> for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Model> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            Ok(ModelBuilder::new(Element::Group, self.src_ref())
                .add_children(self.statements.eval(context)?)?
                .attributes(self.statements.eval(context)?)
                .build())
        })
    }
}

impl Eval<Attributes> for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Attributes> {
        self.statements.eval(context)
    }
}
