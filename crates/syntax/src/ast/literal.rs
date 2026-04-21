// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Span,
    ast::{ItemExtras, Unit},
};

use compact_str::CompactString;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

/// A literal value
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Literal {
    pub span: Span,
    pub extras: ItemExtras,
    pub literal: LiteralKind,
}

impl Literal {
    /// Get the span for the literal
    pub fn span(&self) -> Span {
        self.literal.span()
    }
}

/// The various types of literal values a [`Literal`] can contain
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum LiteralKind {
    Error(LiteralError),
    String(StringLiteral),
    Bool(BoolLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Quantity(QuantityLiteral),
}

impl LiteralKind {
    /// Get the span for the literal
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

/// A string literal, without format expressions
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct StringLiteral {
    pub span: Span,
    pub content: String,
}

/// A boolean literal, either `true` or `false`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct BoolLiteral {
    pub span: Span,
    pub value: bool,
}

/// An integer literal without type
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct IntegerLiteral {
    pub span: Span,
    pub value: i64,
    pub raw: CompactString,
}

/// An float literal without type
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct FloatLiteral {
    pub span: Span,
    pub value: f64,
    pub raw: CompactString,
}

// A float literal with type
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct QuantityLiteral {
    pub span: Span,
    pub value: f64,
    pub raw: CompactString,
    pub unit: Unit,
}

/// An error that can be encountered while parsing literal tokens
#[derive(Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub struct LiteralError {
    pub span: Span,
    pub kind: LiteralErrorKind,
}

#[derive(Debug, Error, PartialEq, Clone)]
#[allow(missing_docs)]
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
