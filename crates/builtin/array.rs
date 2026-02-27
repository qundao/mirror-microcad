// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin array functions.

use microcad_builtin_proc_macros::builtin_mod;

/// Module for  built-in array functions.
///
/// These functions are only supposed to work with [`Value::Array`].
#[builtin_mod]
pub mod array {
    use microcad_lang::{
        diag::PushDiag,
        eval::EvalError,
        parameter,
        resolve::Symbol,
        value::{Value, ValueAccess},
    };

    /// Return the number of elements in an array.
    pub fn len() -> Symbol {
        Symbol::new_builtin_fn(
            "len",
            [].into_iter(),
            &|_params, args, ctx| {
                let arg = args.get_single()?;
                Ok(match &arg.1.value {
                    Value::Array(a) => Value::Integer(a.len() as i64),
                    _ => {
                        ctx.error(arg.1, EvalError::BuiltinError("Value has no count.".into()))?;
                        Value::None
                    }
                })
            },
            None,
        )
    }

    /// Return the first element in an array or string.
    pub fn first() -> Symbol {
        Symbol::new_builtin_fn(
            "first",
            [].into_iter(),
            &|_params, args, ctx| {
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
            },
            None,
        )
    }

    /// Return everything but the first element in an array or string.
    pub fn tail() -> Symbol {
        Symbol::new_builtin_fn(
            "tail",
            [].into_iter(),
            &|_params, args, ctx| {
                let arg = args.get_single()?;
                Ok(match &arg.1.value {
                    Value::Array(a) => a.tail().into(),
                    _ => {
                        ctx.error(arg.1, EvalError::BuiltinError("Value has no tail.".into()))?;
                        Value::None
                    }
                })
            },
            None,
        )
    }

    /// Return reversed version of this array.
    pub fn rev() -> Symbol {
        Symbol::new_builtin_fn(
            "rev",
            [].into_iter(),
            &|_params, args, ctx| {
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
            },
            None,
        )
    }

    /// Return a sorted version of this array
    pub fn sorted() -> Symbol {
        Symbol::new_builtin_fn(
            "sorted",
            [].into_iter(),
            &|_params, args, ctx| {
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
            },
            None,
        )
    }

    /// Check if an array contains an element.
    pub fn contains() -> Symbol {
        Symbol::new_builtin_fn(
            "contains",
            [parameter!(arr), parameter!(x)].into_iter(),
            &|_params, args, ctx| {
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
            },
            None,
        )
    }
}
