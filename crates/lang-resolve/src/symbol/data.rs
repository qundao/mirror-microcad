// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::{Hash, Hasher};

use custom_debug::Debug;
use microcad_lang_base::{ComputedHash, HashId, Identifier, SrcRef, element::Visibility};
use serde::{Deserialize, Serialize};

use crate::symbol::{SymbolHandle, def::SymbolDef};

#[derive(Debug, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct SymbolAttributes {
    //doc: DocBlock,
    //meta_data: HashMap<Identifier, Value>,
}

/// Symbol content
#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct SymbolData {
    /// Symbols id
    pub id: Identifier,

    /// Attributes
    pub attr: SymbolAttributes,

    /// Symbol definition
    pub def: SymbolDef,

    /// Visibility
    pub visibility: Visibility,

    /// Source code reference of the symbol definition
    pub src_ref: SrcRef,

    /// Source code reference of symbol's keyword
    pub keyword_ref: SrcRef,
}

impl SymbolData {
    pub fn get_handle(&self) -> SymbolHandle {
        SymbolHandle(self.computed_hash())
    }
}

impl ComputedHash for SymbolData {
    fn computed_hash(&self) -> HashId {
        let mut hasher = microcad_lang_base::Hasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
