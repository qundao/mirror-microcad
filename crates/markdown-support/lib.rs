// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Markdown support library

use std::{io::Write, path::Path};

pub mod book;
mod md;

use microcad_lang::{
    doc::Doc,
    resolve::*,
    syntax::{
        FunctionDefinition, InitDefinition, ModuleDefinition, SourceFile, WorkbenchDefinition,
    },
};

/// Add an extra `#` to each heading line.
fn indent_header_lines(lines: Vec<String>) -> Vec<String> {
    lines
        .into_iter()
        .map(|s| {
            if s.starts_with("#") {
                format!("#{s}")
            } else {
                s
            }
        })
        .collect()
}

/// Fetch documentation as string with indented headers. (Markdown hack)
fn fetch_doc(doc: &impl Doc) -> String {
    indent_header_lines(doc.doc().fetch_lines()).join("\n")
}

pub trait ToMd {
    fn to_md(&self) -> md::Markdown;
}

impl ToMd for InitDefinition {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# `{}`\n{}", self.to_string(), fetch_doc(self)))
    }
}

impl ToMd for SourceFile {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!(
            "# `{}`\n{}",
            self.filename_as_str(),
            fetch_doc(self)
        ))
    }
}

impl ToMd for FunctionDefinition {
    fn to_md(&self) -> md::Markdown {
        use microcad_lang::doc::Doc;
        md::Markdown::new(&format!(
            "# `{}`\n{}",
            self.id,
            indent_header_lines(self.doc().fetch_lines()).join("\n")
        ))
    }
}

impl ToMd for ModuleDefinition {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# `{}`\n{}", self.id, fetch_doc(self)))
    }
}

impl ToMd for WorkbenchDefinition {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# `{}`\n{}", self.id, fetch_doc(self)))
    }
}

impl ToMd for microcad_lang::builtin::Builtin {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# `{}`\n{}", self.id, fetch_doc(self)))
    }
}

impl ToMd for SymbolDef {
    fn to_md(&self) -> md::Markdown {
        match &self {
            SymbolDef::SourceFile(source_file) => source_file.to_md(),
            SymbolDef::Module(module_definition) => module_definition.to_md(),
            SymbolDef::Workbench(workbench_definition) => workbench_definition.to_md(),
            SymbolDef::Function(function_definition) => function_definition.to_md(),
            SymbolDef::Builtin(builtin) => builtin.to_md(),
            _ => md::Markdown::default(),
        }
    }
}

impl ToMd for Symbol {
    fn to_md(&self) -> md::Markdown {
        // Print one line description of a symbol
        fn get_oneline(symbol: &Symbol) -> String {
            match symbol.doc().fetch_lines().first() {
                Some(line) => format!("`{}`: {line}", symbol.id()),
                None => format!("`{}`", symbol.id()),
            }
        }

        let mut md = self.with_def(|def| def.to_md());

        if let Some(first_section) = md.first_mut() {
            first_section.append(
                self.iter()
                    .map(|symbol| format!("- {}", get_oneline(&symbol)))
                    .collect(),
            );
        }

        md
    }
}

pub trait WriteMdFile: ToMd {
    fn write_md(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        let md = self.to_md();
        file.write_all(md.to_string().as_bytes())
    }
}

impl WriteMdFile for Symbol {}
