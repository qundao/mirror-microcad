// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Doc trait.

use crate::{
    builtin::Builtin,
    lower::ir,
    symbol::{Symbol, SymbolDef},
};

use microcad_lang_base::{Refer, SrcRef};

/// Documentation trait to fetch documentation from a [Symbol].
///
/// The retrieved `DocBlock` struct can processed further to markdown.
/// Depending on the `Symbol`, the returned DocBlock might be empty.
pub trait Doc {
    /// Return block of documentation.
    fn doc(&self) -> ir::DocBlock {
        ir::DocBlock::merge(&self.outer_doc(), &self.inner_doc())
    }

    /// Fetch inner documentation.
    fn inner_doc(&self) -> ir::DocBlock {
        ir::DocBlock::default()
    }

    /// Fetch outer documentation.
    fn outer_doc(&self) -> ir::DocBlock {
        ir::DocBlock::default()
    }
}

impl Doc for ir::DocBlock {
    fn doc(&self) -> ir::DocBlock {
        self.clone()
    }
}

impl Doc for ir::InitDefinition {
    fn outer_doc(&self) -> ir::DocBlock {
        self.doc.clone()
    }

    fn inner_doc(&self) -> ir::DocBlock {
        self.body.inner_doc()
    }
}

impl Doc for ir::StatementList {
    fn inner_doc(&self) -> ir::DocBlock {
        if self.is_empty() {
            ir::DocBlock::default()
        } else {
            let src_ref = SrcRef::merge_all(self.iter());

            ir::DocBlock(Refer::new(
                self.iter()
                    .filter_map(|s| match s {
                        ir::Statement::InnerDocComment(doc) => Some(doc.0.value.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                src_ref,
            ))
        }
    }
}

impl Doc for ir::Body {
    fn inner_doc(&self) -> ir::DocBlock {
        self.statements.inner_doc()
    }
}

impl Doc for ir::ModuleDefinition {
    fn outer_doc(&self) -> ir::DocBlock {
        self.doc.clone()
    }

    fn inner_doc(&self) -> ir::DocBlock {
        self.body
            .as_ref()
            .map(|body| body.inner_doc())
            .unwrap_or_default()
    }
}

impl Doc for ir::FunctionDefinition {
    fn outer_doc(&self) -> ir::DocBlock {
        self.doc.clone()
    }

    fn inner_doc(&self) -> ir::DocBlock {
        self.body.inner_doc()
    }
}

impl Doc for ir::WorkbenchDefinition {
    fn outer_doc(&self) -> ir::DocBlock {
        self.doc.clone()
    }

    fn inner_doc(&self) -> ir::DocBlock {
        self.body.inner_doc()
    }
}

impl Doc for ir::SourceFile {
    fn inner_doc(&self) -> ir::DocBlock {
        self.statements.inner_doc()
    }
}

impl Doc for ir::Assignment {
    fn outer_doc(&self) -> ir::DocBlock {
        self.doc.clone()
    }
}

impl Doc for Builtin {
    fn outer_doc(&self) -> ir::DocBlock {
        match self {
            Builtin::Function(builtin_function) => builtin_function.doc.clone(),
            Builtin::Workbench(builtin_workbench) => builtin_workbench.doc.clone(),
            Builtin::Constant(builtin_constant) => builtin_constant.doc.clone(),
        }
        .unwrap_or_default()
    }
}

impl Doc for SymbolDef {
    fn inner_doc(&self) -> ir::DocBlock {
        match &self {
            SymbolDef::SourceFile(source_file) => source_file.inner_doc(),
            SymbolDef::Module(module_definition) => module_definition.inner_doc(),
            SymbolDef::Workbench(workbench_definition) => workbench_definition.inner_doc(),
            SymbolDef::Function(function_definition) => function_definition.inner_doc(),
            _ => ir::DocBlock::default(),
        }
    }

    fn outer_doc(&self) -> ir::DocBlock {
        match &self {
            SymbolDef::Module(module_definition) => module_definition.outer_doc(),
            SymbolDef::Workbench(workbench_definition) => workbench_definition.outer_doc(),
            SymbolDef::Function(function_definition) => function_definition.outer_doc(),
            SymbolDef::Assignment(assignment) => assignment.outer_doc(),
            SymbolDef::Builtin(builtin) => builtin.outer_doc(),
            //SymbolDef::Constant(..) => todo!(),
            //SymbolDef::Alias(..) => todo!(),
            _ => ir::DocBlock::default(),
        }
    }
}

impl Doc for Symbol {
    fn inner_doc(&self) -> ir::DocBlock {
        self.with_def(|def| def.inner_doc())
    }

    fn outer_doc(&self) -> ir::DocBlock {
        self.with_def(|def| def.outer_doc())
    }
}
