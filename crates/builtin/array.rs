// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin array functions.

use microcad_lang::{diag::*, eval::*, resolve::*, ty::*, value::*};

/// Module for  built-in array functions.
///
/// These functions are only supposed to work with [`Value::Array`].
pub fn array() -> Symbol {
    crate::ModuleBuilder::new("array")
        .symbol(count())
        .symbol(head())
        .symbol(tail())
        .symbol(rev())
        .build()
}

/// Return the count of elements in an array.
fn count() -> Symbol {
    Symbol::new_builtin_fn(
        "count",
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
        &|_| Ok(Type::Integer),
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
                Value::Array(a) if !a.is_empty() => a.head(),
                Value::Array(_) => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value is empty.".into()))?;
                    Value::None
                }
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no head.".into()))?;
                    Value::None
                }
            })
        },
        &|params| {
            if params.len() == 1 {
                if let Type::Array(ty) = params.iter().next().expect("internal error").1.ty() {
                    Ok(*ty)
                } else {
                    todo!("not an array")
                }
            } else {
                todo!("wrong number of parameters")
            }
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
                Value::Array(a) => Value::Array(a.tail()),
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no tail.".into()))?;
                    Value::None
                }
            })
        },
        &|params| {
            if params.len() == 1 {
                Ok(params.iter().next().expect("internal error").1.ty())
            } else {
                todo!("wrong number of parameters")
            }
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
        &|params| Ok(params.iter().next().expect("internal error").1.ty()),
        None,
    )
}
