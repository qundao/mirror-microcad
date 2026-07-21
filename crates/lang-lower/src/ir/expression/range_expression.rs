// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Range expression

use derive_more::Deref;
use microcad_lang_base::SrcRef;
use serde::{Deserialize, Serialize};

use crate::CastInto;

/// Range start.
#[derive(Clone, Debug, Default, Deref, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub struct RangeFirst<EXPR>(pub Box<EXPR>);

impl<EXPR> std::fmt::Display for RangeFirst<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T, EXPR: Serialize> CastInto<RangeFirst<T>> for RangeFirst<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> RangeFirst<T> {
        RangeFirst(Box::new(self.0.cast_into()))
    }
}

/// Range end.
#[derive(Clone, Debug, Default, Deref, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub struct RangeLast<EXPR>(pub Box<EXPR>);

impl<EXPR> std::fmt::Display for RangeLast<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T, EXPR: Serialize> CastInto<RangeLast<T>> for RangeLast<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> RangeLast<T> {
        RangeLast(Box::new(self.0.cast_into()))
    }
}

/// Range expression, e.g. `a..b`.
#[derive(Clone, Debug, Default, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
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

impl<T, EXPR: Serialize> CastInto<RangeExpression<T>> for RangeExpression<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> RangeExpression<T> {
        RangeExpression {
            first: self.first.cast_into(),
            last: self.last.cast_into(),
            src_ref: self.src_ref,
        }
    }
}
