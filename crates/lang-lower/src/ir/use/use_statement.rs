// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement syntax element.

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::ir;

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
    pub visibility: ir::Visibility,
    /// Use declaration
    pub decl: ir::UseDeclaration,
    /// source code reference
    pub src_ref: SrcRef,
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.visibility {
            ir::Visibility::Private | ir::Visibility::PrivateUse(_) => write!(f, "use ")?,
            ir::Visibility::Public => write!(f, "pub use ")?,
            ir::Visibility::Deleted => unreachable!(),
        }
        write!(f, "{}", self.decl)?;
        Ok(())
    }
}
