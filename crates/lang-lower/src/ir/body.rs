// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Body syntax element.

use crate::ir;

use derive_more::Deref;
use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

/// [StatementList] from inside `{}` brackets.
#[derive(Clone, Debug, Default, Deref, SrcReferrer)]
pub struct Body {
    /// Body statements.
    #[deref]
    pub statements: ir::StatementList,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, " {{")?;
        writeln!(f, "{}", self.statements)?;
        writeln!(f, "}}")?;
        Ok(())
    }
}
