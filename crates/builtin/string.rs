// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin string functions.

use microcad_lang::{diag::*, eval::*, resolve::*, ty::*, value::*};

/// Module for built-in string functions.
pub fn string() -> Symbol {
    crate::ModuleBuilder::new("string").symbol(count()).build()
}

/// Return the count of characters in a string.
fn count() -> Symbol {
    Symbol::new_builtin_fn(
        "count",
        [].into_iter(),
        &|_params, args, ctx| {
            let arg = args.get_single()?;
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
        },
        &|_| Ok(Type::Integer),
        None,
    )
}
