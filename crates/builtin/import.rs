// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in import function.

use microcad_builtin_proc_macros::builtin_fn;
use microcad_lang::{builtin::*, eval::*, value::*};
use microcad_lang_base::PushDiag;

/// `__builtin::import` function to import data from files.
#[builtin_fn(filename: String, id: String = String::new())]
pub fn import() -> Symbol {
    |parameter_values, argument_values, context| match ArgumentMatch::find_match(
        argument_values,
        parameter_values,
    ) {
        Ok(arg_map) => {
            let search_paths = context.search_paths().clone();
            context.import(&arg_map, &search_paths)
        }
        Err(err) => {
            context.error(argument_values, err)?;
            Ok(Value::None)
        }
    }
}
