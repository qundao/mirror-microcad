// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Assignment statement syntax elements

use std::rc::Rc;

use microcad_lang_base::{SrcRef, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

use crate::syntax::*;

/// An assignment statement, e.g. `#[aux] s = Sphere(3.0mm);`.
#[derive(Clone, Debug, SrcReferrer)]
pub struct AssignmentStatement {
    /// List of attributes.
    pub attribute_list: AttributeList,
    /// The actual assignment.
    pub assignment: Rc<Assignment>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl TreeDisplay for AssignmentStatement {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Assignment {}", "", self.assignment)
    }
}

impl std::fmt::Display for AssignmentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            write!(f, "{} ", self.attribute_list)?;
        }
        write!(f, "{};", self.assignment)
    }
}
