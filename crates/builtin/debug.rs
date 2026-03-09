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
        eval::{ArgumentMatch, EvalContext, EvalError, EvalResult},
        parameter,
        src_ref::SrcReferrer,
        syntax::{Identifier, QualifiedName},
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

    fn _is_valid(
        src_ref: impl SrcReferrer,
        name: &String,
        context: &mut EvalContext,
    ) -> EvalResult<bool> {
        // Hack split input string and construct a qualified name.
        let name = QualifiedName::new(
            name.split("::").map(|s| Identifier::no_ref(s)).collect(),
            microcad_lang::src_ref::SrcRef(None),
        );
        use microcad_lang::resolve::Lookup;
        Ok(
            match context.lookup(&name, microcad_lang::resolve::LookupTarget::AnyButMethod) {
                Ok(_) => true,
                Err(EvalError::SymbolNotFound(_)) => false,
                Err(err) => {
                    context.error(&src_ref, err)?;
                    false
                }
            }
            .into(),
        )
    }

    /// Check if a qualified name is a valid symbol.
    #[builtin_fn(name: String, message: String = String::new())]
    pub fn is_valid() -> Symbol {
        |params, args, context| {
            match ArgumentMatch::find_match(args, params) {
                Ok(args) => {
                    let name: String = args.get("name");

                    Ok(_is_valid(args, &name, context)?.into())
                }
                Err(err) => {
                    // Called `assert` with no or more than 2 parameters
                    context.error(args, err)?;
                    Ok(Value::None)
                }
            }
        }
    }

    /// Assert if the name string is a valid symbol.
    #[builtin_fn(name: String, message: String = String::new())]
    pub fn assert_valid() -> Symbol {
        |params, args, context| {
            match ArgumentMatch::find_match(args, params) {
                Ok(args) => {
                    let name: String = args.get("name");
                    if !_is_valid(&args, &name, context)? {
                        let message: String = args.get("message");
                        context.error(
                            &args,
                            EvalError::AssertionFailed(if message.is_empty() {
                                String::new()
                            } else {
                                message
                            }),
                        )?;
                    }

                    Ok(Value::None)
                }
                Err(err) => {
                    // Called `assert` with no or more than 2 parameters
                    context.error(args, err)?;
                    Ok(Value::None)
                }
            }
        }
    }

    /// Assert if the name string is an invalid symbol.
    #[builtin_fn(name: String, message: String = String::new())]
    pub fn assert_invalid() -> Symbol {
        |params, args, context| {
            match ArgumentMatch::find_match(args, params) {
                Ok(args) => {
                    let name: String = args.get("name");
                    if _is_valid(&args, &name, context)? {
                        let message: String = args.get("message");
                        context.error(
                            &args,
                            EvalError::AssertionFailed(if message.is_empty() {
                                String::new()
                            } else {
                                message
                            }),
                        )?;
                    }

                    Ok(Value::None)
                }
                Err(err) => {
                    // Called `assert` with no or more than 2 parameters
                    context.error(args, err)?;
                    Ok(Value::None)
                }
            }
        }
    }
}
