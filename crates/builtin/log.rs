// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::builtin_mod;

/// Module for built-in logging.
#[builtin_mod]
#[allow(clippy::module_inception)]
pub mod log {
    use microcad_builtin_proc_macros::builtin_fn;
    use microcad_lang::{diag::*, eval::*, parameter, value::*};

    /// Log error.
    #[builtin_fn(x)]
    pub fn error() -> Symbol {
        |_, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), DiagError> {
                    context.error(
                        args,
                        EvalError::BuiltinError(format!("{value}", value = arg.value)),
                    )
                })?;
            Ok(Value::None)
        }
    }

    /// Log warning.
    #[builtin_fn(x)]
    pub fn warning() -> Symbol {
        |_, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), DiagError> {
                    context.warning(
                        args,
                        EvalError::BuiltinError(format!("{value}", value = arg.value)),
                    )
                })?;
            Ok(Value::None)
        }
    }

    /// Log info message.
    #[builtin_fn(x)]
    pub fn info() -> Symbol {
        |_, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), EvalError> {
                    context.info(args, format!("{value}", value = arg.value));
                    Ok(())
                })?;
            Ok(Value::None)
        }
    }

    /// Log todo message. Will throw an error when reached.
    #[builtin_fn(x)]
    pub fn todo() -> Symbol {
        |_, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), DiagError> {
                    context.error(args, EvalError::Todo(format!("{value}", value = arg.value)))
                })?;
            Ok(Value::None)
        }
    }
}
