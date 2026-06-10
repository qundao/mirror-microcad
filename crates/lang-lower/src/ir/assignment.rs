// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad assignment syntax element

use crate::ir;

use microcad_lang_base::{Identifier, SrcRef};

/// A local assignment specifying an identifier, type and value
#[derive(Clone, Debug)]
pub struct LocalAssignment<EXPR> {
    /// Assignee
    pub(crate) id: Identifier,
    /// Type of the assignee
    pub specified_type: Option<ir::TypeAnnotation>,
    /// Value to assign
    pub expression: EXPR,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<EXPR> std::fmt::Display for LocalAssignment<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use microcad_lang_types::ty::Ty;
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
