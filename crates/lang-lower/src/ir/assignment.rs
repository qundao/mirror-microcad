// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad assignment syntax element

use crate::{CastInto, ir};

use microcad_lang_base::{Identifier, SrcRef};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A local assignment specifying an identifier, type and value
#[skip_serializing_none]
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub struct LocalAssignment<EXPR> {
    /// Assignee
    pub(crate) id: Identifier,
    /// Type of the assignee
    #[serde(skip_serializing_if = "Option::is_none", default)]
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

impl<T, EXPR> CastInto<LocalAssignment<T>> for LocalAssignment<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> LocalAssignment<T> {
        LocalAssignment {
            id: self.id,
            specified_type: self.specified_type,
            expression: self.expression.cast_into(),
            src_ref: self.src_ref,
        }
    }
}
