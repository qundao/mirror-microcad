// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod assert;

pub use assert::*;

use microcad_lang::resolve::*;

/// Module for built-in debugging.
pub fn debug() -> Symbol {
    crate::ModuleBuilder::new("debug")
        .symbol(assert())
        .symbol(assert_eq())
        .symbol(assert_valid())
        .symbol(assert_invalid())
        .build()
}
