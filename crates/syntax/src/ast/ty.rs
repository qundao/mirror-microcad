use crate::Span;
use crate::ast::Identifier;
use compact_str::CompactString;
use indexmap::IndexMap;

#[derive(Debug, PartialEq)]
pub enum Type {
    Single(SingleType),
    Array(ArrayType),
    Tuple(TupleType),
    NamedTuple(NamedTupleType),
}

impl Type {
    pub fn span(&self) -> Span {
        match self {
            Type::Single(ty) => ty.span.clone(),
            Type::Array(ty) => ty.span.clone(),
            Type::Tuple(ty) => ty.span.clone(),
            Type::NamedTuple(ty) => ty.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SingleType {
    pub span: Span,
    pub name: CompactString,
}

#[derive(Debug, PartialEq)]
pub struct ArrayType {
    pub span: Span,
    pub inner: Box<Type>,
}

#[derive(Debug, PartialEq)]
pub struct TupleType {
    pub span: Span,
    pub inner: Vec<Type>,
}

#[derive(Debug, PartialEq)]
pub struct NamedTupleType {
    pub span: Span,
    pub inner: IndexMap<Identifier, Type>,
}
