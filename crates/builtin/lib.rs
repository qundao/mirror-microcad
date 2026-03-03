// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library.
//!
//! The built-in library is provides as µcad module `__builtin` and inserted into a resolve context.

pub mod dirs;
pub mod geo2d;
pub mod geo3d;
pub mod ops;

mod array;
mod color;
mod debug;
mod import;
mod log;
mod math;
mod print;
mod string;

pub use microcad_lang::builtin::{
    Exporter, ExporterAccess, ExporterRegistry, Importer, ImporterRegistry, ModuleBuilder, Symbol,
};

use microcad_lang::{ty::*, value::*};

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
        &|_| Ok(Type::String),
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
        &|_| Ok(Type::String),
        None,
    )
}

/// Build the standard module
pub fn builtin_module() -> Symbol {
    ModuleBuilder::new("__builtin")
        .symbol(debug::debug())
        .symbol(log::log())
        .symbol(array::array())
        .symbol(string::string())
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
