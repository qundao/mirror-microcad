// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod assert;

pub use assert::*;

use microcad_builtin_proc_macros::builtin_mod;
use microcad_lang::resolve::*;

/// Module for built-in debugging.
#[builtin_mod]
pub fn debug() {
    [assert, assert_eq, assert_valid, assert_invalid]
}
