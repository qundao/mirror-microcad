// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

impl CallTrait for BuiltinFunction {
    /// Call builtin function with given parameter.
    ///
    /// # Arguments
    /// - `args`: Function arguments.
    /// - `context`: Execution context.
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        (self.f)(&self.parameters, args, context)
    }
}

impl CallTrait for BuiltinWorkbench {
    /// Call builtin function with given parameter.
    ///
    /// # Arguments
    /// - `args`: Function arguments.
    /// - `context`: Execution context.
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        (self.f)(&self.parameters, args, context)
    }
}

impl CallTrait for Builtin {
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            Builtin::Function(f) => f.call(args, context),
            Builtin::Workbench(w) => w.call(args, context),
            Builtin::Constant(c) => {
                context.error(
                    &microcad_lang_base::SrcRef::none(),
                    EvalError::BuiltinError(format!(
                        "Built-in constant `{}` cannot be called.",
                        c.id()
                    )),
                )?;
                Ok(Value::None)
            }
        }
    }
}
