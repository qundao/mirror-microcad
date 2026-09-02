// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a single markdown file for symbol.

mod to_md;

use std::error::Error;

use crate::DocGen;
use microcad_lang_markdown::WriteToFile;
use microcad_lang_resolve::{SymbolDef, SymbolNodeExt, SymbolNodeRef};
pub(crate) use to_md::ToMd;

/// Markdown generator that generates a markdown documentation file for each source file.
pub struct Md {
    pub output_path: Option<std::path::PathBuf>,
}

impl Md {
    /// Return path
    pub fn symbol_md_file_path<'a>(&self, symbol: SymbolNodeRef<'a>) -> std::path::PathBuf {
        let mut path: std::path::PathBuf =
            symbol.abs_path().iter().map(|id| id.to_string()).collect();
        path.set_extension("md");
        self.output_path.clone().unwrap_or_default().join(path)
    }

    pub fn write_md_file<'a>(&self, symbol: SymbolNodeRef<'a>) -> Result<(), Box<dyn Error>> {
        Ok(symbol
            .to_md()
            .write_to_file(self.symbol_md_file_path(symbol))
            .map_err(Box::new)?)
    }
}

impl DocGen for Md {
    fn doc_gen<'a>(&self, symbol: SymbolNodeRef<'a>) -> Result<(), Box<dyn Error>> {
        symbol
            .descendants()
            .filter(|symbol| matches!(symbol.def(), SymbolDef::Source(_)))
            .try_for_each(|symbol| self.write_md_file(symbol))
    }
}
