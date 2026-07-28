// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Array expressions

use crate::{CastInto, ir};
use derive_more::{Deref, Display};
use microcad_lang_base::{SrcRef, SrcReferrer};
use serde::{Deserialize, Serialize};

/// Inner of an [`ArrayExpression`].
#[derive(Clone, Debug, Display, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub enum ArrayExpressionInner<EXPR> {
    /// List: `a,b,c`.
    List(ir::ListExpression<EXPR>),
    /// Range: `a..b`.
    Range(ir::RangeExpression<EXPR>),
}

impl<T, EXPR> CastInto<ArrayExpressionInner<T>> for ArrayExpressionInner<EXPR>
where
    EXPR: CastInto<T> + Serialize,
{
    fn cast_into(self) -> ArrayExpressionInner<T> {
        use ArrayExpressionInner::*;
        match self {
            List(list_expression) => List(list_expression.cast_into()),
            Range(range_expression) => Range(range_expression.cast_into()),
        }
    }
}

impl<EXPR> SrcReferrer for ArrayExpressionInner<EXPR>
where
    EXPR: SrcReferrer,
{
    fn src_ref(&self) -> SrcRef {
        match &self {
            ArrayExpressionInner::List(expressions) => SrcRef::merge(
                &expressions
                    .0
                    .first()
                    .map(|start| start.src_ref())
                    .unwrap_or_default(),
                &expressions
                    .0
                    .last()
                    .map(|end| end.src_ref())
                    .unwrap_or_default(),
            ),
            ArrayExpressionInner::Range(range_expression) => range_expression.src_ref,
        }
    }
}

/// Array of expressions with common result unit, e.g. `[1+2,4,9]mm`.
#[derive(Clone, Debug, Display, Deref, Hash, PartialEq, Serialize, Deserialize)]
#[display("[{}]{}", inner, unit)]
#[serde(bound(serialize = "Expr: Serialize", deserialize = "Expr: Deserialize<'de>"))]
pub struct ArrayExpression<Expr> {
    /// Expression list.
    #[deref]
    pub inner: ArrayExpressionInner<Expr>,
    /// Unit.
    pub unit: ir::Unit,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<T, EXPR> CastInto<ArrayExpression<T>> for ArrayExpression<EXPR>
where
    EXPR: CastInto<T> + Serialize,
{
    fn cast_into(self) -> ArrayExpression<T> {
        ArrayExpression {
            inner: self.inner.cast_into(),
            unit: self.unit,
            src_ref: self.src_ref,
        }
    }
}
