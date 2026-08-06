// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Creator of work pieces.

//use microcad_package::SymbolId;

use crate::Tuple;
use microcad_lang_base::{HashId, SrcRef};
use serde::{Deserialize, Serialize};

/// Symbol id (TODO Move this `microcad-package`)
#[derive(Debug, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub struct SymbolId {
    library_hash: HashId,
    symbol_id: u64,
}

/// A creator is the symbol
#[derive(Debug, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub struct Creator {
    /// Symbol.
    pub symbol: SymbolId,
    /// Workpiece arguments.
    pub arguments: Tuple,
    /// Hash id
    pub hash_id: HashId,
    /// Symbol ref of the creator
    pub src_ref: SrcRef,
}

impl std::fmt::Display for Creator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.arguments, self.hash_id)
    }
}
