// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

use microcad_lang::{builtin::*, diag::*, eval::*, value::*};

use crate::Symbol;

/// `__builtin::import` function.
pub fn import() -> Symbol {
    Symbol::new_builtin_fn(
        "import",
        [
            parameter!(filename: String),
            parameter!(id: String = String::new()),
        ]
        .into_iter(),
        &|parameter_values, argument_values, context| match ArgumentMatch::find_match(
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
        },
        None,
    )
}
