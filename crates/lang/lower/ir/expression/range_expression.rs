// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Range expression

use derive_more::Deref;
use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::lower::ir;

/// Range start.
#[derive(Clone, Debug, Default, Deref, PartialEq, SrcReferrer)]
pub struct RangeFirst(pub Box<ir::Expression>);

impl std::fmt::Display for RangeFirst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Range end.
#[derive(Clone, Debug, Default, Deref, PartialEq, SrcReferrer)]
pub struct RangeLast(pub Box<ir::Expression>);

impl std::fmt::Display for RangeLast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Range expression, e.g. `a..b`.
#[derive(Clone, Debug, Default, PartialEq, SrcReferrer)]
pub struct RangeExpression {
    /// First value in the range.
    pub first: RangeFirst,
    /// Last value in the range.
    pub last: RangeLast,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for RangeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.first, self.last)
    }
}
