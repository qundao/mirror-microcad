// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Markdown support library

use std::{io::Write, path::Path};

mod md;

use microcad_lang::{
    doc::Doc,
    resolve::*,
    src_ref,
    syntax::{
        DocBlock, FunctionDefinition, InitDefinition, Initialized, ModuleDefinition, SourceFile,
        Statement, StatementList, WorkbenchDefinition,
    },
};

use crate::md::{Markdown, Section};

pub mod book;

pub trait ToMd: microcad_lang::doc::Doc {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&self.doc().fetch_text())
    }
}

impl ToMd for InitDefinition {}

impl ToMd for StatementList {
    fn to_md(&self) -> md::Markdown {
        let section = Section::from_markdown(&self.doc().fetch_text());

        // TODO Add initializers
        // TODO Write constants

        // TODO Write properties

        Markdown(section)
    }
}

impl ToMd for SourceFile {}

impl ToMd for FunctionDefinition {}

impl ToMd for ModuleDefinition {}

impl ToMd for WorkbenchDefinition {
    // TODO: Also add initializers and properties.
}

impl ToMd for microcad_lang::builtin::Builtin {}

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
        self.with_def(|def| def.to_md())
    }
}

pub trait WriteMdFile: ToMd {
    fn write_md(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.to_md().to_string().as_bytes())
    }
}

impl WriteMdFile for Symbol {}
