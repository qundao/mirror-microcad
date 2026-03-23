// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Call related evaluation entities

#[macro_use]
mod argument;
mod call_method;
mod call_trait;

pub use call_method::*;
pub use call_trait::*;
use microcad_lang_base::SrcReferrer;

use crate::{eval::*, symbol::SymbolDef, syntax::*, value::*};

impl Eval<ArgumentValueList> for ArgumentList {
    /// Evaluate into a [`ArgumentValueList`].
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValueList> {
        self.iter()
            .map(|arg| {
                (
                    arg.id.clone().unwrap_or(Identifier::none()),
                    arg.eval(context),
                )
            })
            .map(|(id, arg)| match arg {
                Ok(arg) => Ok((id.clone(), arg)),
                Err(err) => Err(err),
            })
            .collect()
    }
}

impl Eval for Call {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        // find self in symbol table by own name
        let symbol = match context.lookup(&self.name, LookupTarget::Function) {
            Ok(symbol) => symbol,
            Err(err) => {
                context.error(self, err)?;
                return Ok(Value::None);
            }
        };

        // evaluate arguments
        let args: ArgumentValueList = self.argument_list.eval(context)?;

        log::debug!(
            "{call} {name:?}({args:?})",
            name = self.name,
            call = microcad_lang_base::mark!(CALL),
        );

        match context.scope(
            StackFrame::Call {
                symbol: symbol.clone(),
                args: args.clone(),
                src_ref: self.src_ref(),
            },
            |context| {
                symbol.with_def(|def| match def {
                    SymbolDef::Builtin(f) => f.call(&args, context),
                    SymbolDef::Workbench(w) => {
                        if matches!(*w.kind, WorkbenchKind::Operation) {
                            context.error(self, EvalError::CannotCallOperationWithoutWorkpiece)?;
                            Ok(Value::None)
                        } else {
                            Ok(Value::Model(w.call(
                                self.src_ref(),
                                symbol.clone(),
                                &args,
                                context,
                            )?))
                        }
                    }
                    SymbolDef::Function(f) => f.call(&args, context),
                    _ => {
                        context.error(self, EvalError::SymbolCannotBeCalled(symbol.full_name()))?;
                        Ok(Value::None)
                    }
                })
            },
        ) {
            Ok(value) => Ok(value),
            Err(err) => {
                context.error(self, err)?;
                Ok(Value::None)
            }
        }
    }
}
