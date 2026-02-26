// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin string functions.

use microcad_builtin_proc_macros::{builtin_fn, builtin_mod};
use microcad_core::Integer;
use microcad_lang::{diag::PushDiag, eval::EvalError, resolve::Symbol, value::Value};

/// Module for built-in string functions.
#[builtin_mod]
pub fn string() {
    [count];
}

/// Return the count of characters in a string.
#[builtin_fn]
fn count(s: String) -> Integer {
    (s.chars().count() as Integer).into()
}
