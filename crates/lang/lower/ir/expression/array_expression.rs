// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of expression

use crate::lower::ir;
use derive_more::{Deref, DerefMut};
use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_proc_macros::SrcReferrer;

/// Inner of an [`ArrayExpression`].
#[derive(Clone, Debug, PartialEq)]
pub enum ArrayExpressionInner<EXPR = ir::Expression> {
    /// List: `a,b,c`.
    List(ir::ListExpression<EXPR>),
    /// Range: `a..b`.
    Range(ir::RangeExpression<EXPR>),
}

impl std::fmt::Display for ArrayExpressionInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ArrayExpressionInner::List(expressions) => expressions
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                ArrayExpressionInner::Range(range_expression) => range_expression.to_string(),
            }
        )
    }
}

impl SrcReferrer for ArrayExpressionInner {
    fn src_ref(&self) -> SrcRef {
        match &self {
            ArrayExpressionInner::List(expressions) => SrcRef::merge(
                &expressions
                    .first()
                    .map(|start| start.src_ref())
                    .unwrap_or_default(),
                &expressions
                    .last()
                    .map(|end| end.src_ref())
                    .unwrap_or_default(),
            ),
            ArrayExpressionInner::Range(range_expression) => range_expression.src_ref(),
        }
    }
}

/// Array of expressions with common result unit, e.g. `[1+2,4,9]`.
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, SrcReferrer)]
pub struct ArrayExpression<EXPR = ir::Expression> {
    /// Expression list.
    #[deref]
    #[deref_mut]
    pub inner: ArrayExpressionInner<EXPR>,
    /// Unit.
    pub unit: ir::Unit,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for ArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]{}", self.inner, self.unit)
    }
}
