// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{resolve::*, src_ref::*};
use custom_debug::Debug;

/// Symbol content
#[derive(Default, Debug, Clone)]
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

impl SymbolInner {
    pub fn kind_ref(&self) -> Option<SrcRef> {
        match &self.def {
            SymbolDef::Module(m) => Some(m.keyword_ref.clone()),
            SymbolDef::Workbench(wb) => Some(wb.keyword_ref.clone()),
            SymbolDef::Function(f) => Some(f.keyword_ref.clone()),
            _ => None,
        }
    }
}

impl SrcReferrer for SymbolInner {
    fn src_ref(&self) -> SrcRef {
        match &self.def {
            SymbolDef::Root => SrcRef(None),
            SymbolDef::SourceFile(sf) => sf.src_ref(),
            SymbolDef::Module(m) => m.src_ref(),
            SymbolDef::Workbench(wb) => wb.src_ref(),
            SymbolDef::Function(f) => f.src_ref(),
            SymbolDef::Builtin(_) => SrcRef(None),
            SymbolDef::Assignment(a) => a.src_ref(),
            SymbolDef::Constant(_, id, _) | SymbolDef::Argument(id, _) => id.src_ref(),
            SymbolDef::Alias(_, id, _) => id.src_ref(),
            SymbolDef::UseAll(_, name) => name.src_ref(),
            #[cfg(test)]
            SymbolDef::Tester(id) => id.src_ref(),
        }
    }
}
