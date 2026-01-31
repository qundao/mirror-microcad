use crate::Span;
use crate::ast::{ItemExtras, SingleType};
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub extras: ItemExtras,
    pub literal: LiteralKind,
}

impl Literal {
    pub fn span(&self) -> Span {
        self.literal.span()
    }
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Error(LiteralError),
    String(StringLiteral),
    Bool(BoolLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Quantity(QuantityLiteral),
}

impl LiteralKind {
    pub fn span(&self) -> Span {
        match self {
            LiteralKind::Error(lit) => lit.span.clone(),
            LiteralKind::String(lit) => lit.span.clone(),
            LiteralKind::Bool(lit) => lit.span.clone(),
            LiteralKind::Integer(lit) => lit.span.clone(),
            LiteralKind::Float(lit) => lit.span.clone(),
            LiteralKind::Quantity(lit) => lit.span.clone(),
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
    #[error("unclosed string literal")]
    UnclosedString,
    #[error("only numeric literals can be typed")]
    Untypable,
}
