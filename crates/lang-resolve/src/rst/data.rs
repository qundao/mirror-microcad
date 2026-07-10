// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::{Hash, Hasher};

use custom_debug::Debug;
use microcad_lang_base::{ComputedHash, HashId, Identifier, SrcRef, element::Visibility};
use serde::{Deserialize, Serialize};

use crate::rst::def::SymbolDef;

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
            attr: todo!(),
            def: todo!(),
            visibility: todo!(),
            src_ref: todo!(),
            keyword_ref: todo!(),
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
