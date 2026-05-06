// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lowering compile step

pub mod ir;
mod lower;

use microcad_lang_base::{ComputedHash, Hashed, Identifier, Span, SrcRef};

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

#[derive(Clone)]
pub struct LineIndex {
    /// Offset (bytes) the beginning of each line, zero-based
    line_offsets: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> LineIndex {
        let mut line_offsets: Vec<usize> = vec![0];

        let mut offset = 0;

        for c in text.chars() {
            offset += c.len_utf8();
            if c == '\n' {
                line_offsets.push(offset);
            }
        }

        LineIndex { line_offsets }
    }

    /// Returns (line, col) of pos.
    ///
    /// The pos is a byte offset, start from 0, e.g. "ab" is 2, "你好" is 6
    pub fn line_col(&self, input: &str, pos: usize) -> (u32, u32) {
        let line = self.line_offsets.partition_point(|&it| it <= pos) - 1;
        let first_offset = self.line_offsets[line];

        // Get line str from original input, then we can get column offset
        let line_str = &input[first_offset..pos];
        let col = line_str.chars().count();

        ((line + 1) as u32, (col + 1) as u32)
    }
}

pub struct LowerContext<'source> {
    pub source: Hashed<&'source str>,
    line_index: LineIndex,
}

impl<'source> LowerContext<'source> {
    pub fn new(source: &'source str) -> Self {
        LowerContext {
            source: Hashed::new(source),
            line_index: LineIndex::new(source),
        }
    }

    pub fn src_ref(&self, span: &Span) -> SrcRef {
        let (line, col) = self.line_index.line_col(&self.source, span.start);
        SrcRef::new(span.clone(), line, col, self.source.computed_hash())
    }
}

pub trait Lower: Sized {
    type AstNode;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError>;
}
