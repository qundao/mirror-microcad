// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a markdown book from a symbol tree.

use std::io::Write;

use microcad_lang::resolve::{FullyQualify, Symbol, SymbolDef};

use thiserror::Error;

use crate::WriteMdFile;

pub struct BookWriter {
    path: std::path::PathBuf,
}

#[derive(Debug, Error)]
enum BookWriteError {
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl BookWriter {
    fn new(&self, path: impl AsRef<std::path::Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Because this function is tested and imports a built-in file, it has intentionally no error handling.
    fn generate_book_toml_string(&self) -> String {
        let book_toml: toml::Value =
            toml::de::from_str(include_str!("book.toml")).expect("Valid toml");
        toml::ser::to_string(&book_toml).expect("No error")
    }

    /// Generate the toml file for the book
    fn write_book_toml(&self) -> std::io::Result<()> {
        let mut file = std::fs::File::create(self.path.join("book.toml"))?;
        file.write(self.generate_book_toml_string().as_bytes())?;
        Ok(())
    }

    /// Return the path for a symbol.
    ///
    /// For example `std::geo2d::Circle` returns `geo2d/Circle.md`.
    fn symbol_path(symbol: &Symbol) -> std::path::PathBuf {
        let path: std::path::PathBuf = symbol
            .full_name()
            .iter()
            .skip(1)
            .map(|id| id.to_string())
            .collect();
        symbol.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) => path.join("README.md"),
            _ => {
                let mut path = path.clone();
                path.set_extension("md");
                path
            }
        })
    }

    /// Test
    fn write_symbol(&self, symbol: &Symbol) -> Result<(), BookWriteError> {
        symbol.write_md(Self::symbol_path(symbol));
    }

    fn write_summary(&self, symbol: &Symbol) {
        
    }

    pub fn write(&self, symbol: &Symbol) -> Result<(), BookWriteError> {
        self.write_book_toml()?;
        self.write_symbol(&symbol)
    }
}
