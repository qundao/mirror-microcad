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

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Identifier {
    pub span: Span,
    pub name: CompactString,
}

#[derive(Debug)]
pub struct SourceFile {
    pub span: Span,
    pub statements: StatementList,
}

#[derive(Debug, PartialEq, Default)]
pub struct ItemExtras {
    pub leading: Vec<ItemExtra>,
    pub trailing: Vec<ItemExtra>,
}

#[derive(Debug, PartialEq)]
pub enum ItemExtra {
    Comment(Comment),
}