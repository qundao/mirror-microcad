// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement syntax element.

use microcad_lang_base::{SrcRef, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

use crate::syntax::*;

/// Use statement.
///
/// # Example
/// ```ucad
/// use std::*;
/// ```
#[derive(Clone, Debug, SrcReferrer)]
pub struct UseStatement {
    /// SrcRef of the `use` keyword
    pub keyword_ref: SrcRef,
    /// export of use
    pub visibility: Visibility,
    /// Use declaration
    pub decl: UseDeclaration,
    /// source code reference
    pub src_ref: SrcRef,
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.visibility {
            Visibility::Private | Visibility::PrivateUse(_) => write!(f, "use ")?,
            Visibility::Public => write!(f, "pub use ")?,
            Visibility::Deleted => unreachable!(),
        }
        write!(f, "{}", self.decl)?;
        Ok(())
    }
}

impl TreeDisplay for UseStatement {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}UseStatement", "")?;
        depth.indent();
        self.decl.tree_print(f, depth)
    }
}
