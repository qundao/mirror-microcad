// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Markdown support library

mod md;
mod mdbook;

use std::error::Error;

/// Documentation generator for a symbol.
pub trait DocGen {
    fn doc_gen(&self, symbol: &microcad_lang::symbol::Symbol) -> Result<(), Box<dyn Error>>;
}

/// Single markdown generator.
pub use md::Md;

/// mdbook Generator.
pub use mdbook::MdBook;
