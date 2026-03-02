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
mod string;

use microcad_builtin_proc_macros::builtin_mod;
pub use microcad_lang::builtin::{
    Exporter, ExporterAccess, ExporterRegistry, Importer, ImporterRegistry, ModuleBuilder, Symbol,
};

/// µcad standard built-in module.
#[builtin_mod]
pub mod __builtin {
    pub use crate::array::array;
    pub use crate::color::color;
    pub use crate::debug::debug;
    pub use crate::geo2d::geo2d;
    pub use crate::geo3d::geo3d;
    pub use crate::import::import;
    pub use crate::log::log;
    pub use crate::math::math;
    pub use crate::ops::ops;
    pub use crate::string::string;

    use microcad_lang::{diag::PushDiag, eval::EvalError, resolve::Symbol, ty::Ty, value::Value};

    /// Return type of argument.
    pub fn type_of() -> Symbol {
        Symbol::new_builtin_fn(
            "type_of",
            [].into_iter(),
            &|_, args, _| {
                Ok(if let Ok((_, arg)) = args.get_single() {
                    arg.value.ty().to_string().into()
                } else {
                    Value::None
                })
            },
            None,
        )
    }

    /// Convert a value into a string.
    pub fn to_string() -> Symbol {
        Symbol::new_builtin_fn(
            "to_string",
            [].into_iter(),
            &|_, args, _| {
                Ok(if let Ok((_, arg)) = args.get_single() {
                    arg.value.to_string().into()
                } else {
                    Value::None
                })
            },
            None,
        )
    }

    /// Built-in print function.
    pub fn print() -> Symbol {
        Symbol::new_builtin_fn(
            "print",
            [microcad_lang::parameter!(x)].into_iter(),
            &|_params, args, context| {
                args.iter().try_for_each(
                    |(_, arg)| -> Result<(), microcad_lang::eval::EvalError> {
                        context.print(format!("{value}", value = arg.value));
                        Ok(())
                    },
                )?;
                Ok(Value::None)
            },
            None,
        )
    }

    /// Return the count of elements in an array or string.
    ///
    /// Note: This symbol might be deprecated in the future.
    pub fn count() -> Symbol {
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
}

pub use __builtin as builtin_module;

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
