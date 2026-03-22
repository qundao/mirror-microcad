// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Body syntax element.

use derive_more::Deref;
use microcad_lang_base::{SrcRef, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

use crate::syntax::*;

/// [StatementList] from inside `{}` brackets.
#[derive(Clone, Debug, Default, Deref, SrcReferrer)]
pub struct Body {
    /// Body statements.
    #[deref]
    pub statements: StatementList,
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

impl TreeDisplay for Body {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Body:", "")?;
        depth.indent();
        self.statements
            .iter()
            .try_for_each(|s| s.tree_print(f, depth))
    }
}
