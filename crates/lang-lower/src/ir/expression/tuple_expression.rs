// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple expression.

use crate::{CastInto, ir};

use derive_more::Display;
use microcad_lang_base::SrcRef;
use serde::{Deserialize, Serialize};

/// Tuple expression, e.g. `(x=1+2,4,z=9)`.
#[derive(Debug, Display, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[display("({})", args)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub struct TupleExpression<EXPR> {
    /// List of tuple members.
    pub args: ir::ArgumentList<EXPR>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<T, EXPR> CastInto<TupleExpression<T>> for TupleExpression<EXPR>
where
    EXPR: CastInto<T> + Serialize,
{
    fn cast_into(self) -> TupleExpression<T> {
        TupleExpression {
            args: self.args.cast_into(),
            src_ref: self.src_ref,
        }
    }
}
