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

pub struct SourceFile {
    span: Span,
    statements: Vec<Statement>,
}