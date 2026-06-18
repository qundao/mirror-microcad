// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use custom_debug::Debug;
use microcad_lang_base::{SrcRef, SrcReferrer};

use crate::symbol::{Symbol, SymbolDef, SymbolMap};

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
    /// Flag if this symbol was in use
    pub(super) used: std::cell::OnceCell<()>,
}

impl SymbolInner {
    pub fn kind_ref(&self) -> Option<SrcRef> {
        match &self.def {
            SymbolDef::Module(m) => Some(m.keyword_ref),
            SymbolDef::Workbench(wb) => Some(wb.keyword_ref),
            SymbolDef::Function(f) => Some(f.keyword_ref),
            _ => None,
        }
    }
}

impl SrcReferrer for SymbolInner {
    fn src_ref(&self) -> SrcRef {
        match &self.def {
            SymbolDef::Root => SrcRef::none(),
            SymbolDef::SourceFile(sf) => sf.src_ref(),
            SymbolDef::Module(m) => m.src_ref(),
            SymbolDef::Workbench(wb) => wb.src_ref(),
            SymbolDef::Function(f) => f.src_ref(),
            SymbolDef::Builtin(_) => SrcRef::none(),
            SymbolDef::Assignment(a) => a.src_ref(),
            SymbolDef::Value(id, ..) => id.src_ref(),
            SymbolDef::Alias(_, id, _) => id.src_ref(),
            SymbolDef::UseAll(_, name) => name.src_ref(),
            #[cfg(test)]
            SymbolDef::Tester(id) => id.src_ref(),
        }
    }
}
