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
#[serde(bound(serialize = "Expr: Serialize", deserialize = "Expr: Deserialize<'de>"))]
pub struct LocalAssignment<Expr> {
    /// Assignee
    pub id: Identifier,
    /// Type of the assignee
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub specified_type: Option<ir::TypeAnnotation>,
    /// Value to assign
    pub expression: Expr,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<T> LocalAssignment<T> {
    /// Maps the inner expression to a new type while preserving node metadata.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> LocalAssignment<U> {
        LocalAssignment {
            id: self.id,
            specified_type: self.specified_type,
            expression: f(self.expression),
            src_ref: self.src_ref,
        }
    }
}

impl<Expr> std::fmt::Display for LocalAssignment<Expr>
where
    Expr: std::fmt::Display,
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

macro_rules! impl_cast_into {
    ($ty:ident) => {
        impl<Source, Target> CastInto<$ty<Target>> for $ty<Source>
        where
            Source: $crate::CastInto<Target>,
        {
            fn cast_into(self) -> $ty<Target> {
                self.map(CastInto::cast_into)
            }
        }
    };
}

impl_cast_into!(LocalAssignment);
