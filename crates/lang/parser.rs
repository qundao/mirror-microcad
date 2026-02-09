// Copyright © 2024-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Code Parser

#![allow(missing_docs)]

use microcad_syntax::Span;
use crate::{parse::*, src_ref::*};

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
    pub fn line_col(&self, input: &str, pos: usize) -> (usize, usize) {
        let line = self.line_offsets.partition_point(|&it| it <= pos) - 1;
        let first_offset = self.line_offsets[line];

        // Get line str from original input, then we can get column offset
        let line_str = &input[first_offset..pos];
        let col = line_str.chars().count();

        (line + 1, col + 1)
    }
}

pub struct ParseContext<'source> {
    pub source: &'source str,
    pub source_file_hash: u64,
    line_index: LineIndex,
}

impl<'source> ParseContext<'source> {
    pub fn new(source: &'source str) -> Self {
        let source_file_hash = {
            use std::hash::{Hash, Hasher};
            let mut hasher = rustc_hash::FxHasher::default();
            source.hash(&mut hasher);
            hasher.finish()
        };
        ParseContext {
            source,
            source_file_hash,
            line_index: LineIndex::new(source),
        }
    }

    pub fn src_ref(&self, span: &Span) -> SrcRef {
        let (line, col) = self.line_index.line_col(self.source, span.start);
        SrcRef::new(span.clone(), line, col, self.source_file_hash)
    }
}

pub trait FromAst: Sized {
    type AstNode;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError>;
}