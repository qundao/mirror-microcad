// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin string functions.

use microcad_builtin_proc_macros::builtin_mod;

/// Module for built-in string functions.
#[builtin_mod]
#[allow(clippy::module_inception)]
pub mod string {
    use microcad_builtin_proc_macros::builtin_fn;
    use microcad_lang::{parameter, value::Value};
    use microcad_lang_base::PushDiag;

    /// Return the length a string.
    #[builtin_fn(s: String)]
    pub fn len() -> Symbol {
        |_params, args, ctx| {
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
        }
    }

    /// Return the count of characters in a string.
    #[builtin_fn(s: String)]
    pub fn count() -> Symbol {
        |_params, args, ctx| {
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
        }
    }
}
