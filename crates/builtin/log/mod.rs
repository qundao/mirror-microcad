// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{diag::*, eval::*, parameter, resolve::*, ty::*, value::*};

/// Module for built-logging.
pub fn log() -> Symbol {
    crate::ModuleBuilder::new("log")
        .symbol(todo())
        .symbol(error())
        .symbol(warning())
        .symbol(info())
        .build()
}

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
        &|_| Ok(Type::Invalid),
        None,
    )
}

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
        &|_| Ok(Type::Invalid),
        None,
    )
}

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
        &|_| Ok(Type::Invalid),
        None,
    )
}

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
        &|_| Ok(Type::Invalid),
        None,
    )
}
