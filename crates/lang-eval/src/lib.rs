// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of symbols.

mod argument_match;

mod call;
mod eval_context;
mod eval_error;
mod expression;
mod parameter;
mod workbench;

pub use argument_match::*;
pub use call::*;
pub use eval_context::*;
pub use eval_error::*;
pub use parameter::*;

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
