// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Return statement syntax elements.

use microcad_lang_base::{SrcRef, SrcReferrer, TreeDisplay, TreeState};

use crate::syntax::*;

/// Return statement.
#[derive(Clone)]
pub struct ReturnStatement {
    /// SrcRef of the `return` keyword.
    pub keyword_ref: SrcRef,
    /// Return value.
    pub result: Option<Expression>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for ReturnStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(result) = &self.result {
            write!(f, "{result}")
        } else {
            write!(f, microcad_lang_base::invalid_no_ansi!(RESULT))
        }
    }
}

impl std::fmt::Debug for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(result) = &self.result {
            write!(f, "{result:?}")
        } else {
            write!(f, microcad_lang_base::invalid!(RESULT))
        }
    }
}

impl TreeDisplay for ReturnStatement {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}ReturnStatement:", "")?;
        depth.indent();
        if let Some(result) = &self.result {
            result.tree_print(f, depth)?;
        }
        Ok(())
    }
}
