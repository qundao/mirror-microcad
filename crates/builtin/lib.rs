// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library.
//!
//! The built-in library is provides as µcad module `__builtin` and inserted into a resolve context.

pub mod dirs;
pub mod geo2d;
pub mod geo3d;
pub mod ops;

mod color;
mod debug;
mod import;
mod log;
mod math;
mod print;

pub use microcad_lang::builtin::{
    Exporter, ExporterAccess, ExporterRegistry, Importer, ImporterRegistry, ModuleBuilder, Symbol,
};

use microcad_lang::{diag::*, eval::*, ty::Ty, value::*};

/// Return type of argument.
fn type_of() -> Symbol {
    Symbol::new_builtin_fn(
        "type_of",
        [].into_iter(),
        &|_, args, _| {
            if let Ok(arg) = args.get_single() {
                let ty = arg.1.ty();
                return Ok(Value::String(ty.to_string()));
            }
            Ok(Value::None)
        },
        None,
    )
}

/// Return the count of elements in an array or string.
fn count() -> Symbol {
    Symbol::new_builtin_fn(
        "count",
        [].into_iter(),
        &|_params, args, ctx| {
            let arg = args.get_single()?;
            Ok(match &arg.1.value {
                Value::String(s) => Value::Integer(s.chars().count() as i64),
                Value::Array(a) => Value::Integer(a.len() as i64),
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no count.".into()))?;
                    Value::None
                }
            })
        },
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
                Value::String(s) if !s.is_empty() => {
                    Value::String(s.chars().next().unwrap_or_default().to_string())
                }
                Value::Array(a) if !a.is_empty() => a.head(),
                Value::String(_) | Value::Array(_) => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value is empty.".into()))?;
                    Value::None
                }
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no head.".into()))?;
                    Value::None
                }
            })
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
                Value::String(s) => Value::String(s.chars().skip(1).collect()),
                Value::Array(a) => Value::Array(a.tail()),
                _ => {
                    ctx.error(arg.1, EvalError::BuiltinError("Value has no tail.".into()))?;
                    Value::None
                }
            })
        },
        None,
    )
}

/// Convert a value into a string.
fn to_string() -> Symbol {
    Symbol::new_builtin_fn(
        "to_string",
        [].into_iter(),
        &|_, args, _| {
            let (_, arg) = args.get_single()?;
            Ok(Value::String(arg.value.to_string()))
        },
        None,
    )
}

/// Build the standard module
pub fn builtin_module() -> Symbol {
    ModuleBuilder::new("__builtin")
        .symbol(debug::debug())
        .symbol(log::log())
        .symbol(count())
        .symbol(head())
        .symbol(tail())
        .symbol(type_of())
        .symbol(to_string())
        .symbol(print::print())
        .symbol(ops::ops())
        .symbol(math::math())
        .symbol(color::color())
        .symbol(import::import())
        .symbol(geo2d::geo2d())
        .symbol(geo3d::geo3d())
        .build()
}

/// Get built-in importers.
pub fn builtin_importers() -> ImporterRegistry {
    ImporterRegistry::default().insert(microcad_import::toml::TomlImporter)
}

/// Get built-in exporters.
pub fn builtin_exporters() -> ExporterRegistry {
    use microcad_export::*;
    ExporterRegistry::new()
        .insert(svg::SvgExporter)
        .insert(stl::StlExporter)
        .insert(json::JsonExporter)
        .insert(wkt::WktExporter)
}
