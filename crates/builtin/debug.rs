// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::builtin_mod;

/// Module for built-in debugging.
#[builtin_mod]
#[allow(clippy::module_inception)]
pub mod debug {
    use microcad_builtin_proc_macros::builtin_fn;
    use microcad_lang::{
        builtin::ValueAccess,
        diag::PushDiag,
        eval::{ArgumentMatch, EvalError},
        parameter,
        value::Value,
    };

    /// Assertion with an optional message.
    #[builtin_fn(v: Bool, message: String = String::new())]
    pub fn assert() -> Symbol {
        |params, args, context| {
            match ArgumentMatch::find_multi_match(args, params) {
                Ok(multi_args) => {
                    for a in multi_args.args {
                        let v: bool = a.get("v");
                        if !v {
                            let message: String = a.get("message");
                            context.error(
                                args,
                                EvalError::AssertionFailed(if message.is_empty() {
                                    format!("{v}")
                                } else {
                                    message
                                }),
                            )?;
                        }
                    }
                }
                Err(err) => {
                    // Called `assert` with no or more than 2 parameters
                    context.error(args, err)?
                }
            }

            Ok(Value::None)
        }
    }

    fn all_equal<T: PartialEq + std::fmt::Debug>(mut iter: impl Iterator<Item = T>) -> bool {
        if let Some(first) = iter.next() {
            iter.all(|x| x == first)
        } else {
            true
        }
    }

    /// Assert equal.
    #[builtin_fn(array, message: String = String::new())]
    pub fn assert_eq() -> Symbol {
        |params, args, context| {
            match ArgumentMatch::find_multi_match(args, params) {
                Ok(multi_args) => {
                    for array in multi_args.args {
                        let array_value = &array.get_value("array").expect("missing parameter");

                        if let Value::Array(exprs) = array_value {
                            if !all_equal(exprs.iter()) {
                                let message: String = array.get("message");
                                context.error(
                                    args,
                                    EvalError::AssertionFailed(if message.is_empty() {
                                        format!("Values differ: {exprs}")
                                    } else {
                                        "{message}".to_string()
                                    }),
                                )?;
                            }
                        } else {
                            let message: String = array.get("message");
                            context.error(
                                args,
                                EvalError::AssertionFailed(if message.is_empty() {
                                    format!("Invalid: {array_value}")
                                } else {
                                    "{message}".to_string()
                                }),
                            )?;
                        }
                    }
                }
                Err(err) => {
                    // Called `assert` with no or more than 2 parameters
                    context.error(args, err)?
                }
            }

            Ok(Value::None)
        }
    }

    /// Assert that a symbol is valid.
    #[builtin_fn(id: String, message: String = String::new())]
    pub fn assert_valid() -> Symbol {
        |params, args, context| {
            //context.lookup(syntax::QualifiedName::)

            Ok(Value::None)
        }
    }

    /// Assert that a symbol is invalid.
    #[builtin_fn(id: String, message: String = String::new())]
    pub fn assert_invalid() -> Symbol {
        |params, args, context| {
            //context.lookup(syntax::QualifiedName::)

            Ok(Value::None)
        }
    }
}
