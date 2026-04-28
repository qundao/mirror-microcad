// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax definitions and parser for µcad source code.
//!
//! This module includes the components to parse µcad source code into a stream of tokens or abstract syntax tree.
//!
//! - Transform source code into a stream of tokens with [`lex`]
//! - Create an abstract syntax tree from the list of tokens with [`parse`]

use std::ops::Range;

/// Span for tokens or AST nodes, a range of byte offsets from the start of the source
pub type Span = Range<usize>;

/// Abstract syntax tree for µcad files
pub mod ast;
mod parser;
/// Source tokens for µcad files
pub mod tokens;

use microcad_lang_base::{ComputedHash, Hashed, SrcRef, SrcReferrer};
pub use parser::ParseError;

/// An index to retrieve the offsets in a line in O(log(n)).
#[derive(Clone)]
pub struct LineIndex {
    /// Offset (bytes) the beginning of each line, zero-based
    line_offsets: Vec<usize>,
}

impl LineIndex {
    /// Create a new line index from a &str.
    pub fn new(s: &str) -> Self {
        Self {
            line_offsets: std::iter::once(0)
                .chain(s.match_indices('\n').map(|(i, _)| i + 1))
                .collect(),
        }
    }

    /// Returns (line, col) of pos.
    ///
    /// The pos is a byte offset, start from 0, e.g. "ab" is 2, "你好" is 6
    pub fn line_col(&self, input: &str, pos: usize) -> (usize, usize) {
        let line = self.line_offsets.partition_point(|&it| it <= pos) - 1;
        let first_offset = self.line_offsets[line];

        // Get line str from original input, then we can get column offset
        let line_str = &input[first_offset..pos];
        let col = line_str.chars().count();

        (line + 1, col + 1)
    }
}

/// A µcad source with a parse syntax tree with a line index and the hashed original source code.
pub struct Document<'a> {
    /// Computed line index.
    pub line_index: LineIndex,
    /// The original text
    pub text: Hashed<&'a str>,
    /// The syntax tree
    pub ast: ast::Source,
}

impl<'a> SrcReferrer for Document<'a> {
    fn src_ref(&self) -> SrcRef {
        let (line, col) = self.line_index.line_col(&self.text, self.ast.span.start);
        SrcRef::new(self.ast.span.clone(), line, col, self.text.computed_hash())
    }
}

/// Highlevel API to parse directly from a string
pub fn parse(source: &'_ str) -> Result<Document<'_>, Vec<ParseError>> {
    Ok(Document {
        line_index: LineIndex::new(source),
        text: Hashed::new(source),
        ast: parser::parse(&tokens::lex(source).collect::<Vec<_>>())?,
    })
}
