// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Return statement syntax elements.

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::ir;

/// Return statement.
#[derive(Clone, Debug, SrcReferrer)]
pub struct ReturnStatement {
    /// SrcRef of the `return` keyword.
    pub keyword_ref: SrcRef,
    /// Return value.
    pub result: Option<ir::Expression>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(result) = &self.result {
            write!(f, "{result}")
        } else {
            write!(f, "NO RESULT")
        }
    }
}
