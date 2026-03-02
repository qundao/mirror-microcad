// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin string functions.

use microcad_builtin_proc_macros::builtin_mod;

/// Module for built-in string functions.
#[builtin_mod]
#[allow(clippy::module_inception)]
pub mod string {
    use microcad_lang::{diag::PushDiag, parameter, resolve::Symbol, value::Value};

    /// Return the length a string.
    pub fn len() -> Symbol {
        Symbol::new_builtin_fn(
            "len",
            [parameter!(s: String)].into_iter(),
            &|_params, args, ctx| {
                let (_, arg) = args.get_single()?;
                Ok(match &arg.value {
                    Value::String(s) => s.chars().count().into(),
                    _ => {
                        ctx.error(
                            arg,
                            microcad_lang::eval::EvalError::BuiltinError(
                                "Value is not a string.".into(),
                            ),
                        )?;
                        Value::None
                    }
                })
            },
            None,
        )
    }

    /// Return the count of characters in a string.
    pub fn count() -> Symbol {
        Symbol::new_builtin_fn(
            "count",
            [parameter!(s: String)].into_iter(),
            &|_params, args, ctx| {
                let (_, arg) = args.get_single()?;
                Ok(match &arg.value {
                    Value::String(s) => s.chars().count().into(),
                    _ => {
                        ctx.error(
                            arg,
                            microcad_lang::eval::EvalError::BuiltinError(
                                "Value is not a string.".into(),
                            ),
                        )?;
                        Value::None
                    }
                })
            },
            None,
        )
    }
}
