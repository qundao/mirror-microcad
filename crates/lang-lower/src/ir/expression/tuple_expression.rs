// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple expression.

use crate::ir;

use microcad_lang_base::SrcRef;
use serde::Serialize;

/// Tuple expression, e.g. `(x=1+2,4,z=9)`.
#[derive(Debug, PartialEq, Serialize)]
#[serde(bound(serialize = "EXPR: Serialize"))]
pub struct TupleExpression<EXPR> {
    /// List of tuple members.
    pub args: ir::ArgumentList<EXPR>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<EXPR> std::fmt::Display for TupleExpression<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.args)?;
        Ok(())
    }
}
