// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use custom_debug::Debug;
use microcad_lang_base::{Identifier, SrcRef, SrcReferrer, element::Visibility};

use crate::{Symbol, Symbols, symbol::def::SymbolDef};

#[derive(Debug)]
pub struct SymbolAttributes;

/// Symbol content
#[derive(Default, Debug, Clone)]
pub(super) struct SymbolInner {
    pub id: Identifier,

    /// Attributes
    pub attr: SymbolAttributes,

    /// Symbol definition
    pub def: SymbolDef,

    pub visibility: Visibility,

    pub src_ref: SrcRef,

    pub keyword_ref: SrcRef,

    /// Symbol's parent
    #[debug(skip)]
    pub parent: Option<Symbol>,
    /// Symbol's children
    pub children: Symbols,
}
