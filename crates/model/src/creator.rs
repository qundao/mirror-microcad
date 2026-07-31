// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Creator of work pieces.

//use microcad_package::SymbolId;

use microcad_lang_base::HashId;
use microcad_lang_types::Tuple;
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
}
