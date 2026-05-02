// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument value evaluation entity

use microcad_lang_base::{PushDiag, SrcRef};

use crate::{eval::*, model::Model, symbol::SymbolDef, syntax::*};

/// Trait for calling methods of values
pub trait CallMethod<T = Value> {
    /// Evaluate method call into a value (if possible)
    ///
    /// - `name`: Name of the method
    /// - `args`: Arguments for the method
    /// - `context`: Evaluation context
    fn call_method(
        &self,
        id: &QualifiedName,
        args: &ArgumentValueList,
        context: &mut EvalContext,
    ) -> EvalResult<T>;
}

impl CallMethod for Array {
    fn call_method(
        &self,
        id: &QualifiedName,
        _: &ArgumentValueList,
        context: &mut EvalContext,
    ) -> EvalResult<Value> {
        Ok(
            match id.single_identifier().expect("Single id").id().as_str() {
                "count" => self.len().into(),
                "first" | "head" => self.first(), // Keep head method as deprecated method.
                "last" => self.last(),
                "tail" => self.tail().into(),
                "rev" => self.rev().into(),
                "sorted" => self.sorted().into(),
                "all_equal" => self.all_equal().into(),
                "is_ascending" => self.is_ascending().into(),
                "is_descending" => self.is_descending().into(),
                _ => {
                    context.error(id, EvalError::UnknownMethod(id.clone()))?;
                    Value::None
                }
            },
        )
    }
}

impl CallMethod<Option<Model>> for Model {
    fn call_method(
        &self,
        name: &QualifiedName,
        args: &ArgumentValueList,
        context: &mut EvalContext,
    ) -> EvalResult<Option<Model>> {
        match context.lookup(name, LookupTarget::Method) {
            Ok(symbol) => context.scope(
                StackFrame::Call {
                    symbol: symbol.clone(),
                    args: args.clone(),
                    src_ref: SrcRef::merge(name, args),
                },
                |context| {
                    symbol.with_def(|def| match def {
                        SymbolDef::Workbench(workbench_definition) => {
                            let model = workbench_definition.call(
                                SrcRef::merge(name, args),
                                symbol.clone(),
                                args,
                                context,
                            )?;

                            Ok::<_, EvalError>(Some(model.replace_input_placeholders(self)))
                        }
                        SymbolDef::Builtin(builtin) => match builtin.call(args, context)? {
                            Value::Model(model) => Ok(Some(model.replace_input_placeholders(self))),
                            value => panic!("Builtin call returned {value} but no models."),
                        },
                        _ => {
                            context.error(name, EvalError::SymbolCannotBeCalled(name.clone()))?;
                            Ok(None)
                        }
                    })
                },
            ),
            Err(err) => {
                context.error(name, err)?;
                Ok(None)
            }
        }
    }
}

impl CallMethod for Value {
    fn call_method(
        &self,
        id: &QualifiedName,
        args: &ArgumentValueList,
        context: &mut EvalContext,
    ) -> EvalResult<Value> {
        match self {
            Value::Integer(_) => eval_todo!(context, id, "call_method for Integer"),
            Value::Quantity(_) => eval_todo!(context, id, "call_method for Quantity"),
            Value::Bool(_) => eval_todo!(context, id, "call_method for Bool"),
            Value::String(_) => eval_todo!(context, id, "call_method for String"),
            Value::Tuple(_) => eval_todo!(context, id, "call_method for Tuple"),
            Value::Matrix(_) => eval_todo!(context, id, "call_method for Matrix"),
            Value::Array(array) => array.call_method(id, args, context),
            Value::Model(model) => Ok(model
                .call_method(id, args, context)?
                .map(Value::Model)
                .unwrap_or_default()),
            _ => {
                context.error(id, EvalError::UnknownMethod(id.clone()))?;
                Ok(Value::None)
            }
        }
    }
}
