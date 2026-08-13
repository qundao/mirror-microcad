// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Markdown support library

mod md;
mod mdbook;

/// Single markdown generator.
pub use md::Md;

/// mdbook Generator.
pub use mdbook::MdBook;

use microcad_package::SymbolNodeRef;
use std::error::Error;

/// Documentation generator for a symbol.
pub trait DocGen<'a> {
    fn doc_gen(&self, symbol: SymbolNodeRef<'a>) -> Result<(), Box<dyn Error>>;
}
