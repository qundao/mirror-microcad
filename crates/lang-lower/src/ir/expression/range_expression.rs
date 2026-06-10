// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Range expression

use derive_more::Deref;
use microcad_lang_base::SrcRef;

/// Range start.
#[derive(Clone, Debug, Default, Deref, PartialEq)]
pub struct RangeFirst<EXPR>(pub Box<EXPR>);

impl<EXPR> std::fmt::Display for RangeFirst<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Range end.
#[derive(Clone, Debug, Default, Deref, PartialEq)]
pub struct RangeLast<EXPR>(pub Box<EXPR>);

impl<EXPR> std::fmt::Display for RangeLast<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Range expression, e.g. `a..b`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RangeExpression<EXPR> {
    /// First value in the range.
    pub first: RangeFirst<EXPR>,
    /// Last value in the range.
    pub last: RangeLast<EXPR>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<EXPR> std::fmt::Display for RangeExpression<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.first, self.last)
    }
}
