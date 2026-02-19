// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Markdown support library

use std::{io::Write, path::Path};

pub mod book;
mod md;

use microcad_lang::{
    resolve::*,
    syntax::{
        FunctionDefinition, InitDefinition, ModuleDefinition, SourceFile, StatementList,
        WorkbenchDefinition,
    },
};

pub trait ToMd {
    fn to_md(&self) -> md::Markdown;
}

impl ToMd for InitDefinition {
    fn to_md(&self) -> md::Markdown {
        use microcad_lang::doc::Doc;
        md::Markdown::new(&format!(
            "# `{}`\n{}",
            self.to_string(),
            self.doc().fetch_text()
        ))
    }
}

impl ToMd for SourceFile {
    fn to_md(&self) -> md::Markdown {
        use microcad_lang::doc::Doc;
        md::Markdown::new(&format!(
            "# `{}`\n{}",
            self.filename_as_str(),
            self.doc().fetch_text()
        ))
    }
}

impl ToMd for FunctionDefinition {
    fn to_md(&self) -> md::Markdown {
        use microcad_lang::doc::Doc;
        md::Markdown::new(&format!("# `{}`\n{}", self.id, self.doc().fetch_text()))
    }
}

impl ToMd for ModuleDefinition {
    fn to_md(&self) -> md::Markdown {
        use microcad_lang::doc::Doc;
        md::Markdown::new(&format!("# `{}`\n{}", self.id, self.doc().fetch_text()))
    }
}

impl ToMd for WorkbenchDefinition {
    fn to_md(&self) -> md::Markdown {
        use microcad_lang::doc::Doc;
        md::Markdown::new(&format!("# `{}`\n{}", self.id, self.doc().fetch_text()))
    }
}

impl ToMd for microcad_lang::builtin::Builtin {
    fn to_md(&self) -> md::Markdown {
        use microcad_lang::doc::Doc;
        md::Markdown::new(&format!("# `{}`\n{}", self.id, self.doc().fetch_text()))
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
        self.with_def(|def| def.to_md())
    }
}

pub trait WriteMdFile: ToMd {
    fn write_md(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        let md = self.to_md();
        println!("{md}");
        file.write_all(md.to_string().as_bytes())
    }
}

impl WriteMdFile for Symbol {}
