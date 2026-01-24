use crate::Span;
use crate::ast::SingleType;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Error(LiteralError),
    String(StringLiteral),
    Bool(BoolLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Quantity(QuantityLiteral),
}

impl Literal {
    pub fn span(&self) -> Span {
        match self {
            Literal::Error(lit) => lit.span.clone(),
            Literal::String(lit) => lit.span.clone(),
            Literal::Bool(lit) => lit.span.clone(),
            Literal::Integer(lit) => lit.span.clone(),
            Literal::Float(lit) => lit.span.clone(),
            Literal::Quantity(lit) => lit.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    pub span: Span,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct BoolLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub span: Span,
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub struct FloatLiteral {
    pub span: Span,
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub struct QuantityLiteral {
    pub span: Span,
    pub value: f64,
    pub ty: SingleType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralError {
    pub span: Span,
    pub kind: LiteralErrorKind,
}

#[derive(Debug, Error, PartialEq)]
pub enum LiteralErrorKind {
    #[error(transparent)]
    Float(#[from] ParseFloatError),
    #[error(transparent)]
    Int(#[from] ParseIntError),
}
