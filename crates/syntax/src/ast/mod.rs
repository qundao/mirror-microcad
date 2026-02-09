// Copyright © 2026 The µcad authors <info@ucad.xyz>
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

/// A µcad source file
#[derive(Debug)]
#[allow(missing_docs)]
pub struct SourceFile {
    pub span: Span,
    pub statements: StatementList,
}

/// Non-syntactic extras that can be attached to many ast nodes
#[derive(Debug, PartialEq, Default)]
#[allow(missing_docs)]
pub struct ItemExtras {
    pub leading: Vec<ItemExtra>,
    pub trailing: Vec<ItemExtra>,
}

#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum ItemExtra {
    Comment(Comment),
}