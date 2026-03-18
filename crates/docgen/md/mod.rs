// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a single markdown file for symbol.

mod markdown;
mod section;
mod to_md;

pub use markdown::Markdown;
pub use section::Section;
pub use to_md::ToMd;

use crate::DocGen;
use microcad_builtin::Symbol;
use microcad_lang::symbol::SymbolDef;

/// Markdown generator that generates a markdown documentation file for each source file.
pub struct Md {
    pub output_path: Option<std::path::PathBuf>,
}

impl Md {
    /// Return path
    pub fn symbol_md_file_path(&self, symbol: &Symbol) -> std::path::PathBuf {
        let mut path: std::path::PathBuf =
            symbol.full_name().iter().map(|id| id.to_string()).collect();
        path.set_extension("md");
        self.output_path.clone().unwrap_or_default().join(path)
    }

    pub fn write_md_file(&self, symbol: &Symbol) -> std::io::Result<()> {
        symbol.to_md().write(self.symbol_md_file_path(symbol))
    }
}

impl DocGen for Md {
    fn doc_gen(&self, symbol: &Symbol) -> std::io::Result<()> {
        symbol
            .riter()
            .filter(|symbol| symbol.with_def(|def| matches!(def, SymbolDef::SourceFile(_))))
            .try_for_each(|symbol| self.write_md_file(&symbol))
    }
}
