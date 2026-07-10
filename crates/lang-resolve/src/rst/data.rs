// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::{Hash, Hasher};

use custom_debug::Debug;
use microcad_lang_base::{ComputedHash, HashId, Id, Identifier, SrcRef, element::Visibility};
use microcad_lang_lower::ir::DocBlock;
use serde::{Deserialize, Serialize};

use crate::{
    SymbolRef,
    rst::{ResolvedName, UnresolvedName},
};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolDataHandle(microcad_lang_base::HashId);

#[derive(Debug, Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct SymbolAttributes {
    //doc: DocBlock,
    //meta_data: HashMap<Identifier, Value>,
}

/// Symbol content
#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolData {
    pub id: Id,
    /// Doc
    pub doc: DocBlock,

    /// Visibility
    pub visibility: Visibility,

    /// Source code reference of the symbol definition
    pub src_ref: SrcRef,

    /// Source code reference of symbol's keyword
    pub keyword_ref: SrcRef,
}

impl ComputedHash for SymbolData {
    fn computed_hash(&self) -> HashId {
        let mut hasher = microcad_lang_base::Hasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
