// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a single markdown file for symbol.

mod to_md;

use std::error::Error;

use crate::DocGen;
use microcad_builtin::Symbol;
use microcad_lang::symbol::SymbolDef;
pub(crate) use to_md::ToMd;

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

    pub fn write_md_file(&self, symbol: &Symbol) -> Result<(), Box<dyn Error>> {
        Ok(symbol
            .to_md()
            .save(self.symbol_md_file_path(symbol))
            .map_err(|err| Box::new(err))?)
    }
}

impl DocGen for Md {
    fn doc_gen(&self, symbol: &Symbol) -> Result<(), Box<dyn Error>> {
        symbol
            .riter()
            .filter(|symbol| symbol.with_def(|def| matches!(def, SymbolDef::SourceFile(_))))
            .try_for_each(|symbol| self.write_md_file(&symbol))
    }
}
