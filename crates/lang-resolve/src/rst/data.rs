// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::{Hash, Hasher};

use custom_debug::Debug;
use microcad_lang_base::{ComputedHash, HashId, Identifier, SrcRef, element::Visibility};
use serde::{Deserialize, Serialize};

use crate::{
    SymbolRef,
    rst::{ResolvedName, UnresolvedName, def::SymbolDef},
};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolDataHandle(microcad_lang_base::HashId);

#[derive(Debug, Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct SymbolAttributes {
    //doc: DocBlock,
    //meta_data: HashMap<Identifier, Value>,
}

/// Symbol content
#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "NAME: Serialize", deserialize = "NAME: Deserialize<'de>"))]
pub struct SymbolData<NAME: Serialize> {
    /// Attributes
    pub attr: SymbolAttributes,

    /// Symbol definition
    pub def: SymbolDef<NAME>,

    /// Visibility
    pub visibility: Visibility,

    /// Source code reference of the symbol definition
    pub src_ref: SrcRef,

    /// Source code reference of symbol's keyword
    pub keyword_ref: SrcRef,
}

impl<NAME: Serialize> Clone for SymbolData<NAME> {
    fn clone(&self) -> Self {
        Self {
            attr: self.attr.clone(),
            def: SymbolDef::Root,
            visibility: self.visibility.clone(),
            src_ref: self.src_ref,
            keyword_ref: self.keyword_ref,
        }
    }
}

impl<NAME: Serialize + Hash> SymbolData<NAME> {
    pub fn get_handle(&self) -> SymbolDataHandle {
        SymbolDataHandle(self.computed_hash())
    }
}

impl<NAME: Serialize + Hash> ComputedHash for SymbolData<NAME> {
    fn computed_hash(&self) -> HashId {
        let mut hasher = microcad_lang_base::Hasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

fn resolve<'rst>(
    symbol: &SymbolRef<'rst, SymbolData<UnresolvedName>>,
    def: SymbolDef<UnresolvedName>,
) -> SymbolDef<ResolvedName> {
    match def {
        SymbolDef::Root => SymbolDef::Root,
        SymbolDef::SourceFile(source_file) => SymbolDef::SourceFile(source_file),
        SymbolDef::InlineModule(inline_module) => SymbolDef::InlineModule(inline_module),
        SymbolDef::FileModule => todo!(),
        SymbolDef::Workbench => todo!(),
        SymbolDef::Function(function) => todo!(),
        SymbolDef::Constant(constant) => SymbolDef::Constant(constant.resolve(symbol)),
        SymbolDef::Builtin => todo!(),
        SymbolDef::Alias => todo!(),
        SymbolDef::Wildcard => todo!(),
    }
}

impl<'rst> SymbolRef<'rst, SymbolData<UnresolvedName>> {
    pub fn resolve_data(&self) -> SymbolData<ResolvedName> {
        let data = self.data.clone();

        SymbolData {
            attr: data.attr,
            def: resolve(&self, data.def),
            visibility: data.visibility,
            src_ref: data.src_ref,
            keyword_ref: data.keyword_ref,
        }
    }
}
