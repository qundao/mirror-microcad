// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a markdown book from a symbol tree.

use std::io::Write;

use microcad_lang::{
    builtin::Builtin,
    resolve::{FullyQualify, Symbol, SymbolDef},
};

use crate::{DocGen, md::ToMd};

/// Mdbook generator.
///
/// Read mdbook documentation: https://rust-lang.github.io/mdBook/
pub struct MdBook {
    /// Output path.
    pub path: std::path::PathBuf,
}

impl MdBook {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Because this function is tested and imports a built-in file, it has intentionally no error handling.
    fn generate_book_toml_string(&self) -> String {
        let book_toml: toml::Value =
            toml::de::from_str(include_str!("book.toml")).expect("Valid toml");
        let str = toml::ser::to_string(&book_toml).expect("No error");
        format!(
            r#"# Copyright © 2026 The µcad authors <info@ucad.xyz>
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# NOTE: Auto-generated code. 
# This markdown book has been generated from µcad source via `microcad-docgen`.
# Changes in the book might be overwritten.
{str}
"#
        )
    }

    /// Generate the toml file for the book
    fn write_book_toml(&self) -> std::io::Result<()> {
        let mut file = std::fs::File::create(self.path.join("book.toml"))?;
        file.write_all(self.generate_book_toml_string().as_bytes())?;
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

    fn _generate_summary(
        &self,
        writer: &mut impl std::fmt::Write,
        symbol: &Symbol,
        depth: usize,
    ) -> std::fmt::Result {
        fn entry(
            writer: &mut impl std::fmt::Write,
            id: impl std::fmt::Display,
            path: impl AsRef<std::path::Path>,
            depth: usize,
        ) -> std::fmt::Result {
            writeln!(
                writer,
                "{:indent$}- [`{id}`]({path})",
                "",
                indent = 2 * depth,
                path = path.as_ref().display()
            )
        }

        fn recurse<'a>(
            self_: &MdBook,
            writer: &mut impl std::fmt::Write,
            symbols: impl IntoIterator<Item = &'a Symbol>,
            depth: usize,
        ) -> std::fmt::Result {
            symbols
                .into_iter()
                .try_for_each(|symbol| self_._generate_summary(writer, symbol, depth))
        }

        let path = Self::symbol_path(symbol);

        entry(writer, symbol.id(), path, depth)?;
        let depth = depth + 1;

        let children: Vec<_> = symbol.iter().filter(|symbol| symbol.is_public()).collect();

        let modules: Vec<_> = children
            .iter()
            .filter(|symbol| {
                symbol.with_def(|def| {
                    matches!(def, SymbolDef::SourceFile(..) | SymbolDef::Module(..))
                })
            })
            .collect();

        if !modules.is_empty() {
            recurse(self, writer, modules.into_iter(), depth)?;
        }

        // All workbenches (including built-ins) are in separate file.
        let workbenches: Vec<_> = children
            .iter()
            .filter(|symbol| {
                symbol.with_def(|def| match def {
                    SymbolDef::Workbench(_) | SymbolDef::Builtin(Builtin::Workbench(_)) => true,
                    _ => false,
                })
            })
            .collect();

        if !workbenches.is_empty() {
            recurse(self, writer, workbenches.into_iter(), depth)?;
        }

        Ok(())
    }

    fn generate_summary(
        &self,
        writer: &mut impl std::fmt::Write,
        symbol: &Symbol,
    ) -> std::fmt::Result {
        writeln!(writer, "# Summary")?;
        writeln!(writer)?;
        self._generate_summary(writer, symbol, 0)
    }

    fn write_symbol(&self, symbol: &Symbol) -> std::io::Result<()> {
        symbol.riter().try_for_each(|symbol| {
            let path = &self.path.join("src").join(Self::symbol_path(&symbol));
            std::fs::create_dir_all(path.parent().expect("A parent"))?;
            symbol.with_def(|def| match def {
                SymbolDef::SourceFile(_)
                | SymbolDef::Module(_)
                | SymbolDef::Workbench(_)
                | SymbolDef::Builtin(Builtin::Workbench(_)) => symbol.to_md().write(path),
                _ => Ok(()),
            })
        })
    }

    fn write_summary(&self, symbol: &Symbol) -> std::io::Result<()> {
        // 1. Create the SUMMARY.md file
        let mut file = std::fs::File::create(self.path.join("src").join("SUMMARY.md"))?;

        // 2. We use a String as a buffer because generate_summary requires std::fmt::Write
        let mut buffer = String::new();

        // 3. Call generate_summary. We map the fmt::Error to an io::Error.

        self.generate_summary(&mut buffer, symbol)
            .map_err(std::io::Error::other)?;
        file.write_all(buffer.as_bytes())?;
        Ok(())
    }
}

impl DocGen for MdBook {
    fn doc_gen(&self, symbol: &microcad_lang::resolve::Symbol) -> std::io::Result<()> {
        std::fs::create_dir_all(self.path.join("src"))?;

        self.write_book_toml()?;
        self.write_summary(symbol)?;
        self.write_symbol(symbol)
    }
}
