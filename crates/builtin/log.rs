// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::builtin_mod;

/// Module for built-in logging.
#[builtin_mod]
pub mod log {
    use microcad_lang::{diag::*, eval::*, parameter, resolve::*, value::*};

    /// Log error.
    pub fn error() -> Symbol {
        Symbol::new_builtin_fn(
            "error",
            [parameter!(x)].into_iter(),
            &|_, args, context| {
                args.iter()
                    .try_for_each(|(_, arg)| -> Result<(), DiagError> {
                        context.error(
                            args,
                            EvalError::BuiltinError(format!("{value}", value = arg.value)),
                        )
                    })?;
                Ok(Value::None)
            },
            None,
        )
    }

    /// Log warning.
    pub fn warning() -> Symbol {
        Symbol::new_builtin_fn(
            "warning",
            [parameter!(x)].into_iter(),
            &|_, args, context| {
                args.iter()
                    .try_for_each(|(_, arg)| -> Result<(), DiagError> {
                        context.warning(
                            args,
                            EvalError::BuiltinError(format!("{value}", value = arg.value)),
                        )
                    })?;
                Ok(Value::None)
            },
            None,
        )
    }

    /// Log info message.
    pub fn info() -> Symbol {
        Symbol::new_builtin_fn(
            "info",
            [parameter!(x)].into_iter(),
            &|_, args, context| {
                args.iter()
                    .try_for_each(|(_, arg)| -> Result<(), EvalError> {
                        context.info(args, format!("{value}", value = arg.value));
                        Ok(())
                    })?;
                Ok(Value::None)
            },
            None,
        )
    }

    /// Log todo message. Will throw an error when reached.
    pub fn todo() -> Symbol {
        Symbol::new_builtin_fn(
            "todo",
            [parameter!(x)].into_iter(),
            &|_, args, context| {
                args.iter()
                    .try_for_each(|(_, arg)| -> Result<(), DiagError> {
                        context.error(args, EvalError::Todo(format!("{value}", value = arg.value)))
                    })?;
                Ok(Value::None)
            },
            None,
        )
    }
}
