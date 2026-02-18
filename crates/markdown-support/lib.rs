// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Markdown support library

use std::{io::Write, path::Path};

use markdown::{ParseOptions, mdast::Heading};
use microcad_lang::{
    doc::Doc,
    resolve::*,
    src_ref,
    syntax::{
        DocBlock, FunctionDefinition, InitDefinition, Initialized, ModuleDefinition, SourceFile,
        Statement, StatementList, WorkbenchDefinition,
    },
};

mod book;

/// Trait to generate markdown
pub trait ToMdAst: microcad_lang::doc::Doc {
    fn to_mdast(&self) -> Result<markdown::mdast::Node, markdown::message::Message> {
        markdown::to_mdast(&self.doc().fetch_text(), &ParseOptions::default())
    }
}

impl ToMdAst for DocBlock {
    fn to_mdast(&self) -> Result<markdown::mdast::Node, markdown::message::Message> {
        markdown::to_mdast(&self.doc().fetch_text(), &ParseOptions::default())
    }
}

impl ToMdAst for InitDefinition {}

impl ToMdAst for StatementList {
    fn to_mdast(&self) -> Result<markdown::mdast::Node, markdown::message::Message> {
        use markdown::mdast::*;

        let mut children = Vec::new();

        children.push(self.doc().to_mdast()?);

        // TODO Write constants

        // TODO Write properties

        // Write initializers

        children.push(Node::Heading(Heading {
            children: vec![],
            position: None,
            depth: 1,
        }));
        children.push(Node::Text(Text {
            value: "Initializers".into(),
            position: None,
        }));
        children.extend(self.0.iter().filter_map(|stmt| match stmt {
            Statement::Init(init_definition) => Some(init_definition.to_mdast().expect("No error")),
            _ => None,
        }));

        Ok(Node::Root(Root {
            children,
            position: None,
        }))
    }
}

impl ToMdAst for SourceFile {}

impl ToMdAst for FunctionDefinition {}

impl ToMdAst for ModuleDefinition {}

impl ToMdAst for WorkbenchDefinition {
    // TODO: Also add initializers and properties.
}

impl ToMdAst for microcad_lang::builtin::Builtin {}

impl ToMdAst for SymbolDef {
    fn to_mdast(&self) -> Result<markdown::mdast::Node, markdown::message::Message> {
        use markdown::mdast::*;

        match &self {
            SymbolDef::Root => Ok(Node::Root(Root {
                children: vec![],
                position: None,
            })),
            SymbolDef::SourceFile(source_file) => source_file.to_mdast(),
            SymbolDef::Module(module_definition) => module_definition.to_mdast(),
            SymbolDef::Workbench(workbench_definition) => workbench_definition.to_mdast(),
            SymbolDef::Function(function_definition) => function_definition.to_mdast(),
            SymbolDef::Builtin(builtin) => builtin.to_mdast(),
            _ => todo!(),
        }
    }
}

impl ToMdAst for Symbol {
    fn to_mdast(&self) -> Result<markdown::mdast::Node, markdown::message::Message> {
        self.with_def(|def| def.to_mdast())
    }
}

pub trait WriteMdFile: ToMdAst {
    fn write_md(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.to_mdast().expect("No error").to_string().as_bytes())
    }
}

impl WriteMdFile for Symbol {}
