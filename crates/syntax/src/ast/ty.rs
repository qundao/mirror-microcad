// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Span;
use crate::ast::Identifier;
use compact_str::CompactString;

/// The possible types
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Type {
    Single(SingleType),
    Array(ArrayType),
    Tuple(TupleType),
}

impl Type {
    /// Get the span of the type
    pub fn span(&self) -> Span {
        match self {
            Type::Single(ty) => ty.span.clone(),
            Type::Array(ty) => ty.span.clone(),
            Type::Tuple(ty) => ty.span.clone(),
        }
    }
}

/// A type for a single numeric value
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct SingleType {
    pub span: Span,
    pub name: CompactString,
}

/// An array type
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ArrayType {
    pub span: Span,
    pub inner: Box<Type>,
}

/// A tuple type
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct TupleType {
    pub span: Span,
    pub inner: Vec<(Option<Identifier>, Type)>,
}
