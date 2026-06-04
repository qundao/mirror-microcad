// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of symbols.

mod argument_match;
mod attribute;
mod body;
mod call;
mod eval_context;
mod eval_error;
mod expression;
mod format_string;
mod function;

mod init;
mod literal;
mod locals;
mod parameter;
mod source_file;
mod statements;
mod tuple;
mod workbench;

pub use argument_match::*;
pub use attribute::*;
pub use call::*;
pub use eval_context::*;
pub use eval_error::*;
pub use parameter::*;

use locals::*;
use microcad_lang_base::PushDiag;

use crate::{lower::ir, resolve::*, ty::*, value::*};

/// Evaluation trait.
///
/// The return type `T` defines to which output type the type is evaluated.
/// Usually, these output types are used in specific context:
///
/// | Return type `T`     | Context / Scope                                     | Return value on error   | Description                                                             |
/// | ------------------- | --------------------------------------------------- | ----------------------- | ----------------------------------------------------------------------- |
/// | `()`                | [`Assignment`].                                     | `()`                    | An assignment returns nothing but alters the symbol table.              |
/// |                     |                                                     |                         |
/// | `Value`             | Function calls, module statements,                  | `Value::None`           | These trait implementations are   
/// |                     | parameter lists, argument lists.                    |                         | mostly used when evaluating functions.
/// |                     |                                                     |                         |
/// | `Option<Model>`     | Workbenches, object bodies, source files, if.       | `None`                  | Something is evaluated into a single model.                              |
/// |                     |                                                     |                         |
/// | `Models`            | Statement, statement list, body, multiplicities.    | `Models::default()`     | A collection of models . |
pub trait Eval<T = Value> {
    /// Evaluate a syntax element into a type `T`.
    fn eval(&self, context: &mut EvalContext) -> EvalResult<T>;
}

impl ir::MethodCall {
    /// Evaluate method call.
    ///
    /// Examples:
    /// ```microcad
    /// assert([2.0, 2.0].all_equal(), "All elements in this list must be equal.");
    /// ```
    fn eval(&self, context: &mut EvalContext, lhs: &ir::Expression) -> EvalResult<Value> {
        let value: Value = lhs.eval(context)?;
        if let Value::Model(model) = &value {
            if model.has_no_output() {
                context.warning(&lhs, EvalError::EmptyModelExpression)?;
            }
        }
        let args = self.argument_list.eval(context)?;
        value.call_method(&self.name, &args, context)
    }
}

/// Like `todo!()` but within a evaluation context
///
/// emits a diagnostic error instead of panicking.
#[macro_export]
macro_rules! eval_todo {
    ($context: ident, $refer: ident, $($arg:tt)*) => {{
        $context.error($refer, EvalError::Todo(format_args!($($arg)*).to_string()))?;
        Ok(Value::None)
    }}
}

pub use eval_todo;
