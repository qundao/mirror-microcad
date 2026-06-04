// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad value related evaluation entities

use microcad_lang_base::PushDiag;

use crate::{eval::*, lower::ir};

impl Eval<ArgumentValue> for ir::Argument {
    /// Evaluate `Argument` and return `ArgumentValue`
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValue> {
        use crate::lower::SingleIdentifier;

        Ok(ArgumentValue::new(
            match self.expression.eval(context) {
                Ok(value) => value,
                Err(err) => {
                    context.error(self, err)?;
                    Value::None
                }
            },
            self.expression.single_identifier().cloned(),
            self.src_ref.clone(),
        ))
    }
}
