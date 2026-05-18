// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lowering compile step

pub mod ir;

#[allow(clippy::module_inception)]
mod lower;

use microcad_lang_base::{ComputedHash, Hashed, Identifier, LineIndex, Span, SrcRef};

pub use lower::{LowerError, LowerErrorsWithSource, LowerResult};

/// Check if the element only includes one identifier
pub trait SingleIdentifier {
    /// If the element only includes one identifier, return it
    fn single_identifier(&self) -> Option<&Identifier>;

    /// Returns true if the element only includes a single identifier.
    fn is_single_identifier(&self) -> bool {
        self.single_identifier().is_some()
    }
}

/// Identifier accessor.
pub trait Identifiable {
    /// Get clone of the identifier.
    fn id(&self) -> Identifier {
        self.id_ref().clone()
    }

    /// Get reference to the identifier.
    fn id_ref(&self) -> &Identifier;

    /// Get identifier as string.
    fn id_as_str(&self) -> &str {
        self.id_ref().0.as_str()
    }
}

/// Interface for elements which have *initializers*.
pub trait Initialized<'a> {
    /// return iterator of body statements.
    fn statements(&'a self) -> std::slice::Iter<'a, ir::Statement>;

    /// Return iterator over all initializers.
    fn inits(&'a self) -> ir::Inits<'a>
    where
        Self: std::marker::Sized,
    {
        ir::Inits::new(self)
    }
}

pub struct LowerContext<'source> {
    pub source: Hashed<&'source str>,
    line_index: LineIndex,
    line_offset: u32,
}

impl<'source> LowerContext<'source> {
    pub fn new(source: &'source str) -> Self {
        LowerContext {
            source: Hashed::new(source),
            line_index: LineIndex::new(source),
            line_offset: 0,
        }
    }

    pub fn with_line_offset(self, line_offset: u32) -> Self {
        Self {
            source: self.source,
            line_index: self.line_index,
            line_offset,
        }
    }

    pub fn src_ref(&self, span: &Span) -> SrcRef {
        self.line_index
            .src_ref(self.source.value(), span, self.source.computed_hash())
            .with_line_offset(self.line_offset)
    }
}

pub trait Lower: Sized {
    type AstNode;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError>;
}
