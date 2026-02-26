// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin string functions.

use microcad_builtin_proc_macros::builtin_mod;

/// Module for built-in string functions.
#[builtin_mod]
pub mod string {
    use microcad_builtin_proc_macros::builtin_fn;
    use microcad_core::Integer;
    use microcad_lang::{diag::PushDiag, resolve::Symbol, value::Value};

    /// Return the count of characters in a string.
    #[builtin_fn]
    pub fn count(s: String) -> Integer {
        (s.chars().count() as Integer).into()
    }
}
