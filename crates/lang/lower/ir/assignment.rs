// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad assignment syntax element

use crate::{lower::ir, ty::*};

use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_proc_macros::{Identifiable, SrcReferrer};

/// A local assignment specifying an identifier, type and value
#[derive(Clone, Debug, SrcReferrer, Identifiable)]
pub struct LocalAssignment<EXPR = ir::Expression> {
    /// Assignee
    pub(crate) id: Identifier,
    /// Type of the assignee
    pub specified_type: Option<ir::TypeAnnotation>,
    /// Value to assign
    pub expression: EXPR,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl std::fmt::Display for LocalAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(
                f,
                "{id}: {ty} = {expr}",
                id = self.id,
                ty = t.ty(),
                expr = self.expression
            ),
            None => write!(f, "{id} = {expr}", id = self.id, expr = self.expression),
        }
    }
}
