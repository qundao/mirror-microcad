// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin string functions.

use microcad_builtin_proc_macros::{builtin_fn, builtin_mod};
use microcad_lang::{diag::PushDiag, eval::EvalError, resolve::Symbol, value::Value};

/// Module for built-in string functions.
#[builtin_mod]
pub fn string() {
    [count];
}

/// Return the count of characters in a string.
#[builtin_fn]
fn count() {
    Ok(match &arg.1.value {
        Value::String(s) => Value::Integer(s.chars().count() as i64),
        _ => {
            ctx.error(
                arg.1,
                EvalError::BuiltinError("Value is not a string.".into()),
            )?;
            Value::None
        }
    })
}
