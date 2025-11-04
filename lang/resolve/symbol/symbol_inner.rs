// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{resolve::*, src_ref::*, syntax::*};
use custom_debug::Debug;

/// Symbol content
#[derive(Debug, Clone)]
pub(super) struct SymbolInner {
    /// Symbol definition
    pub(super) def: SymbolDef,
    /// Symbol's parent
    #[debug(skip)]
    pub(super) parent: Option<Symbol>,
    /// Symbol's children
    pub(super) children: SymbolMap,
    /// Flag if this symbol has been checked after resolving
    pub(super) checked: std::cell::OnceCell<()>,
    /// Flag if this symbol was in use
    pub(super) used: std::cell::OnceCell<()>,
}

impl Default for SymbolInner {
    fn default() -> Self {
        Self {
            def: SymbolDef::SourceFile(SourceFile::default().into()),
            parent: Default::default(),
            children: Default::default(),
            checked: Default::default(),
            used: Default::default(),
        }
    }
}

impl SrcReferrer for SymbolInner {
    fn src_ref(&self) -> SrcRef {
        match &self.def {
            SymbolDef::SourceFile(source_file) => source_file.src_ref(),
            SymbolDef::Module(module) => module.src_ref(),
            SymbolDef::Workbench(workbench) => workbench.src_ref(),
            SymbolDef::Function(function) => function.src_ref(),
            SymbolDef::Builtin(_) => SrcRef(None),
            SymbolDef::Constant(_, identifier, _)
            | SymbolDef::ConstExpression(_, identifier, _)
            | SymbolDef::Argument(identifier, _) => identifier.src_ref(),
            SymbolDef::Alias(_, identifier, _) => identifier.src_ref(),
            SymbolDef::UseAll(_, name) => name.src_ref(),
            #[cfg(test)]
            SymbolDef::Tester(id) => id.src_ref(),
        }
    }
}
