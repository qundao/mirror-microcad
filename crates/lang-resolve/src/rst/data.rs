// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::Hash;

use custom_debug::Debug;
use microcad_lang_base::{Id, SrcRef, element::Visibility};
use microcad_lang_lower::ir::DocBlock;
use serde::{Deserialize, Serialize};

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
