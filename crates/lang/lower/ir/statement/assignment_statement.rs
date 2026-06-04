// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Assignment statement syntax elements

use std::rc::Rc;

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::lower::ir;

/// An assignment statement, e.g. `#[aux] s = Sphere(3.0mm);`.
#[derive(Clone, Debug, SrcReferrer)]
pub struct LocalAssignmentStatement {
    /// List of attributes.
    pub attribute_list: ir::AttributeList,
    /// The actual assignment.
    pub assignment: Rc<ir::LocalAssignment>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for LocalAssignmentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            write!(f, "{} ", self.attribute_list)?;
        }
        write!(f, "{};", self.assignment)
    }
}
