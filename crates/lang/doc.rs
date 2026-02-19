// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Doc trait.

use crate::{
    builtin::Builtin,
    resolve::*,
    src_ref::{Refer, SrcRef},
    syntax::*,
};

/// Documentation trait to fetch documentation from a [Symbol].
///
/// The retrieved `DocBlock` struct can processed further to markdown.
/// Depending on the `Symbol`, the returned DocBlock might be empty.
pub trait Doc {
    /// Return block of documentation.
    fn doc(&self) -> DocBlock {
        DocBlock::merge(&self.outer_doc(), &self.inner_doc())
    }

    /// Fetch inner documentation.
    fn inner_doc(&self) -> DocBlock {
        DocBlock::default()
    }

    /// Fetch outer documentation.
    fn outer_doc(&self) -> DocBlock {
        DocBlock::default()
    }
}

impl Doc for DocBlock {
    fn doc(&self) -> DocBlock {
        self.clone()
    }
}

impl Doc for InitDefinition {
    fn outer_doc(&self) -> DocBlock {
        self.doc.as_ref().cloned().unwrap_or_default()
    }

    fn inner_doc(&self) -> DocBlock {
        self.body.inner_doc()
    }
}

impl Doc for StatementList {
    fn inner_doc(&self) -> DocBlock {
        if self.is_empty() {
            DocBlock::default()
        } else {
            let src_ref = SrcRef::merge_all(self.iter());

            DocBlock(Refer::new(
                self.iter()
                    .filter_map(|s| match s {
                        Statement::InnerDocComment(doc) => Some(doc.0.value.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                src_ref,
            ))
        }
    }
}

impl Doc for Body {
    fn inner_doc(&self) -> DocBlock {
        self.statements.inner_doc()
    }
}

impl Doc for ModuleDefinition {
    fn outer_doc(&self) -> DocBlock {
        self.doc.as_ref().cloned().unwrap_or_default()
    }

    fn inner_doc(&self) -> DocBlock {
        self.body
            .as_ref()
            .map(|body| body.inner_doc())
            .unwrap_or_default()
    }
}

impl Doc for FunctionDefinition {
    fn outer_doc(&self) -> DocBlock {
        self.doc.as_ref().cloned().unwrap_or_default()
    }

    fn inner_doc(&self) -> DocBlock {
        self.body.inner_doc()
    }
}

impl Doc for WorkbenchDefinition {
    fn outer_doc(&self) -> DocBlock {
        self.doc.as_ref().cloned().unwrap_or_default()
    }

    fn inner_doc(&self) -> DocBlock {
        self.body.inner_doc()
    }
}

impl Doc for SourceFile {
    fn inner_doc(&self) -> DocBlock {
        self.statements.inner_doc()
    }
}

impl Doc for Assignment {
    fn outer_doc(&self) -> DocBlock {
        self.doc.as_ref().cloned().unwrap_or_default()
    }
}

impl Doc for Builtin {
    fn outer_doc(&self) -> DocBlock {
        self.doc.as_ref().cloned().unwrap_or_default()
    }
}

impl Doc for SymbolDef {
    fn inner_doc(&self) -> DocBlock {
        match &self {
            SymbolDef::SourceFile(source_file) => source_file.inner_doc(),
            SymbolDef::Module(module_definition) => module_definition.inner_doc(),
            SymbolDef::Workbench(workbench_definition) => workbench_definition.inner_doc(),
            SymbolDef::Function(function_definition) => function_definition.inner_doc(),
            _ => DocBlock::default(),
        }
    }

    fn outer_doc(&self) -> DocBlock {
        match &self {
            SymbolDef::Module(module_definition) => module_definition.outer_doc(),
            SymbolDef::Workbench(workbench_definition) => workbench_definition.outer_doc(),
            SymbolDef::Function(function_definition) => function_definition.outer_doc(),
            SymbolDef::Assignment(assignment) => assignment.outer_doc(),
            SymbolDef::Builtin(builtin) => builtin.outer_doc(),
            SymbolDef::Constant(..) => todo!(),
            SymbolDef::Alias(..) => todo!(),
            _ => DocBlock::default(),
        }
    }
}

impl Doc for Symbol {
    fn inner_doc(&self) -> DocBlock {
        self.with_def(|def| def.inner_doc())
    }

    fn outer_doc(&self) -> DocBlock {
        self.with_def(|def| def.outer_doc())
    }
}
