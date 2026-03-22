// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Expression statement syntax elements

use microcad_lang_base::{SrcRef, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

use crate::syntax::*;

/// An assignment statement, e.g. `#[aux] s = Sphere(3.0mm);`.
#[derive(Clone, Debug, SrcReferrer)]
pub struct ExpressionStatement {
    /// Optional attributes.
    pub attribute_list: AttributeList,
    /// The actual expression.
    pub expression: Expression,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl TreeDisplay for ExpressionStatement {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        self.expression.tree_print(f, depth)
    }
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            write!(f, "{} ", self.attribute_list)?;
        }
        write!(f, "{};", self.expression)
    }
}
