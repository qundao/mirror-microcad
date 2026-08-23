// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Markdown support library

mod builtin;
mod md;
mod mdbook;

/// Single markdown generator.
pub use md::Md;

/// mdbook Generator.
pub use mdbook::MdBook;

/// builtin documentation generator.
pub use builtin::BuiltinMdbook;

use microcad_package::SymbolNodeRef;
use std::error::Error;

/// Documentation generator for a symbol.
pub trait DocGen {
    fn doc_gen<'a>(&self, symbol: SymbolNodeRef<'a>) -> Result<(), Box<dyn Error>>;
}
