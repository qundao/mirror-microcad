// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax definitions and parser for µcad source code.
//!
//! This module includes the components to parse µcad source code into a stream of tokens or abstract syntax tree.
//!
//! - Transform source code into a stream of tokens with [`lex`]
//! - Create an abstract syntax tree from the list of tokens with [`parse`]

/// Abstract syntax tree for µcad files
pub mod ast;
mod parser;

mod source;

/// Source tokens for µcad files
pub mod tokens;

use microcad_lang_base::{ComputedHash, Diagnostics, Hashed, LineIndex, Span, SrcRef, Url};
pub use parser::ParseError;

pub use parser::parsers;

/// Context for parsing.
pub struct ParseContext<'source> {
    /// The source url
    pub url: Url,
    /// Line index
    line_index: LineIndex,
    /// A line offset (e.g. used when source is parsed from markdown code snippets).
    line_offset: u32,
    /// The source code to be parsed.
    source: Hashed<&'source str>,
}

impl<'source> ParseContext<'source> {
    /// Create a new parse context.
    pub fn new(source: &'source str) -> Self {
        let source = Hashed::new(source);
        Self {
            url: microcad_lang_base::virtual_url(&format!("source_{}", source.computed_hash())),
            line_index: LineIndex::new(&source),
            line_offset: 0,
            source,
        }
    }

    /// Add a URL to the parse context.
    pub fn with_url(self, url: Url) -> Self {
        Self {
            url,
            source: self.source,
            line_index: self.line_index,
            line_offset: self.line_offset,
        }
    }

    /// Add a line offset the parse context.
    pub fn with_line_offset(self, line_offset: u32) -> Self {
        Self {
            url: self.url,
            source: self.source,
            line_index: self.line_index,
            line_offset,
        }
    }

    /// Create a source code reference from a span.
    pub fn src_ref(&self, span: &Span) -> SrcRef {
        self.line_index
            .src_ref(self.source.value(), span, self.source.computed_hash())
            .with_line_offset(self.line_offset)
    }
}

/// Parse trait.
pub trait Parse: Sized {
    /// Parse from a context.
    ///
    /// The context also contains the source string.
    fn parse(context: &ParseContext) -> Result<Self, Diagnostics>;
}

/// API to parse directly from a string
pub fn parse(source: &str) -> Result<ast::Program, Vec<ParseError>> {
    parser::parse(&tokens::lex(source).collect::<Vec<_>>())
}

pub use source::Source;
