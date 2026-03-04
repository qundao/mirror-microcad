// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin array functions.

use microcad_builtin_proc_macros::builtin_mod;

/// Module for  built-in array functions.
///
/// These functions are only supposed to work with [`Value::Array`].
#[builtin_mod]
#[allow(clippy::module_inception)]
pub mod array {
    use microcad_builtin_proc_macros::builtin_fn;
    use microcad_lang::{
        diag::PushDiag,
        eval::EvalError,
        parameter,
        value::{Value, ValueAccess},
    };

    /// Return the number of elements in an array.
    #[builtin_fn(x)]
    pub fn len() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) => Value::Integer(a.len() as i64),
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no count.".into()))?;
                    Value::None
                }
            })
        }
    }

    /// Return the count of elements in an array or string.
    ///
    /// Note: This symbol might be deprecated in the future.
    #[builtin_fn(x)]
    pub fn count() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::String(s) => Value::Integer(s.chars().count() as i64),
                Value::Array(a) => Value::Integer(a.len() as i64),
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no count.".into()))?;
                    Value::None
                }
            })
        }
    }

    /// Return the first element in an array or string.
    #[builtin_fn(x)]
    pub fn first() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) if !a.is_empty() => a.first(),
                Value::Array(_) => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value is empty.".into()))?;
                    Value::None
                }
                _ => {
                    ctx.error(
                        arg.1,
                        EvalError::BuiltinError("Value is not an array.".into()),
                    )?;
                    Value::None
                }
            })
        }
    }

    /// Return everything but the first element in an array or string.
    #[builtin_fn(x)]
    pub fn last() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) if !a.is_empty() => a.last(),
                Value::Array(_) => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value is empty.".into()))?;
                    Value::None
                }
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no tail.".into()))?;
                    Value::None
                }
            })
        }
    }

    /// Return the first element in an array or string.
    ///
    /// Note this function is supposed to be deprecated in the future.
    #[builtin_fn(x)]
    pub fn head() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) if !a.is_empty() => a.first(),
                Value::Array(_) => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value is empty.".into()))?;
                    Value::None
                }
                _ => {
                    ctx.error(
                        arg.1,
                        EvalError::BuiltinError("Value is not an array.".into()),
                    )?;
                    Value::None
                }
            })
        }
    }

    /// Return everything but the first element in an array or string.
    #[builtin_fn(x)]
    pub fn tail() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) => a.tail().into(),
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no tail.".into()))?;
                    Value::None
                }
            })
        }
    }

    /// Return reversed version of this array.
    #[builtin_fn(x)]
    pub fn rev() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) => a.rev().into(),
                _ => {
                    ctx.error(
                        arg.1,
                        EvalError::BuiltinError("Value is not an array.".into()),
                    )?;
                    Value::None
                }
            })
        }
    }

    /// Return a sorted version of this array
    #[builtin_fn(x)]
    pub fn sorted() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) => a.sorted().into(),
                _ => {
                    ctx.error(
                        arg.1,
                        EvalError::BuiltinError("Value is not an array.".into()),
                    )?;
                    Value::None
                }
            })
        }
    }

    /// Check if all items are sorted in ascending order.
    #[builtin_fn(x)]
    pub fn is_ascending() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) => a.is_ascending().into(),
                _ => {
                    ctx.error(
                        arg.1,
                        EvalError::BuiltinError("Value is not an array.".into()),
                    )?;
                    Value::None
                }
            })
        }
    }

    /// Check if all items are sorted in descending order.
    #[builtin_fn(x)]
    pub fn is_descending() -> Symbol {
        |_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) => a.is_descending().into(),
                _ => {
                    ctx.error(
                        arg.1,
                        EvalError::BuiltinError("Value is not an array.".into()),
                    )?;
                    Value::None
                }
            })
        }
    }

    /// Check if an array contains an element.
    #[builtin_fn(arr, x)]
    pub fn contains() -> Symbol {
        |_params, args, ctx| {
            let arr = match args.get_value("arr") {
                Err(_) => args
                    .get_by_index(0)
                    .map(|(_, arg)| arg.value.clone())
                    .unwrap_or_default(),
                Ok(arr) => arr.clone(),
            };
            let x = match args.get_value("x") {
                Err(_) => args
                    .get_by_index(1)
                    .map(|(_, arg)| arg.value.clone())
                    .unwrap_or_default(),
                Ok(arr) => arr.clone(),
            };

            Ok(match arr {
                Value::Array(a) => a.contains(&x).into(),
                _ => {
                    ctx.error(
                        args,
                        EvalError::BuiltinError("Value is not an array.".into()),
                    )?;
                    Value::None
                }
            })
        }
    }
}
