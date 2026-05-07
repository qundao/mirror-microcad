// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad assignment syntax element

use crate::{lower::ir, ty::*};

use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_proc_macros::{Identifiable, SrcReferrer};

/// Assignment specifying an identifier, type and value
#[derive(Clone, Debug, SrcReferrer, Identifiable)]
pub struct Assignment {
    /// Documentation.
    pub doc: ir::DocBlock,
    /// Value's visibility
    pub visibility: ir::Visibility,
    /// Assignee qualifier
    pub qualifier: ir::Qualifier,
    /// Assignee
    pub(crate) id: Identifier,
    /// Type of the assignee
    pub specified_type: Option<ir::TypeAnnotation>,
    /// Value to assign
    pub expression: ir::Expression,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Assignment {
    /// Get qualifier (makes `pub` => `pub const`)
    pub fn qualifier(&self) -> ir::Qualifier {
        match self.visibility {
            ir::Visibility::Private | ir::Visibility::PrivateUse(_) => self.qualifier,
            ir::Visibility::Public => ir::Qualifier::Const,
            ir::Visibility::Deleted => unreachable!(),
        }
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(
                f,
                "{vis}{qual}{id}: {ty} = {expr}",
                vis = self.visibility,
                qual = self.qualifier,
                id = self.id,
                ty = t.ty(),
                expr = self.expression
            ),
            None => write!(
                f,
                "{vis}{qual}{id} = {expr}",
                vis = self.visibility,
                qual = self.qualifier,
                id = self.id,
                expr = self.expression
            ),
        }
    }
}
