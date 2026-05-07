// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::lower::ir;

use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

/// Function definition
#[derive(Clone, Debug, Identifiable)]
pub struct FunctionDefinition {
    /// SrcRef of the `fn` keyword
    pub keyword_ref: SrcRef,
    /// Documentation.
    pub doc: ir::DocBlock,
    /// Visibility
    pub visibility: ir::Visibility,
    /// Name of the function
    pub(crate) id: ir::Identifier,
    /// Function signature
    pub signature: ir::FunctionSignature,
    /// Function body
    pub body: ir::Body,
}

impl SrcReferrer for FunctionDefinition {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl std::fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {}{}", self.id, self.signature)
    }
}
