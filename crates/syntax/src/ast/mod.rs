// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod expression;
mod literal;
mod statement;
mod ty;

use crate::Span;
use compact_str::CompactString;

pub use expression::*;
pub use literal::*;
pub use statement::*;
pub use ty::*;

/// A µcad identifier
#[derive(Debug, PartialEq, Hash, Eq)]
#[allow(missing_docs)]
pub struct Identifier {
    pub span: Span,
    pub name: CompactString,
}

impl Dummy for Identifier {
    fn dummy(span: Span) -> Self {
        Self {
            span,
            name: CompactString::default(),
        }
    }
}

/// A µcad source file
#[derive(Debug)]
#[allow(missing_docs)]
pub struct Source {
    pub span: Span,
    pub statements: StatementList,
}

/// Non-syntactic extras that can be attached to many ast nodes
#[derive(Clone, Debug, PartialEq, Default)]
#[allow(missing_docs)]
pub struct ItemExtras {
    pub leading: LeadingExtras,
    pub trailing: TrailingExtras,
}

/// Extras that occur *before* a syntax element.
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(missing_docs)]
pub struct TrailingExtras(pub Vec<ItemExtra>);

/// Extras that occur *after* a syntax element.
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(missing_docs)]
pub struct LeadingExtras(pub Vec<ItemExtra>);

#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum ItemExtra {
    Comment(Comment),
    Whitespace(String),
}

/// Return a dummy of this syntax element.
///
/// Used for recovery.
pub(crate) trait Dummy {
    fn dummy(span: Span) -> Self;
}
