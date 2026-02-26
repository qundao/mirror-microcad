// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin array functions.

use microcad_lang::{diag::PushDiag, eval::EvalError, resolve::Symbol, value::Value};

/// Module for built-logging.
pub fn array() -> Symbol {
    crate::ModuleBuilder::new("array")
        .symbol(count())
        .symbol(head())
        .symbol(tail())
        .symbol(rev())
        .build()
}

/// Return the count of elements in an array or string.
fn count() -> Symbol {
    Symbol::new_builtin_fn(
        "count",
        [].into_iter(),
        &|_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::String(s) => Value::Integer(s.chars().count() as i64),
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
fn head() -> Symbol {
    Symbol::new_builtin_fn(
        "head",
        [].into_iter(),
        &|_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::String(s) if !s.is_empty() => {
                    Value::String(s.chars().next().unwrap_or_default().to_string())
                }
                Value::Array(a) if !a.is_empty() => a.head(),
                Value::String(_) | Value::Array(_) => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value is empty.".into()))?;
                    Value::None
                }
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no head.".into()))?;
                    Value::None
                }
            })
        },
        None,
    )
}

/// Return everything but the first element in an array or string.
fn tail() -> Symbol {
    Symbol::new_builtin_fn(
        "tail",
        [].into_iter(),
        &|_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::String(s) => Value::String(s.chars().skip(1).collect()),
                Value::Array(a) => Value::Array(a.tail()),
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
fn rev() -> Symbol {
    Symbol::new_builtin_fn(
        "rev",
        [].into_iter(),
        &|_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::Array(a) => Value::Array(a.rev()),
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
