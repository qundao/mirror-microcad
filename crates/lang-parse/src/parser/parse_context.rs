// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse context for parsing specific types.

use microcad_lang_base::{ComputedHash, Hashed, LineIndex, Span, SrcRef, Url};

/// Context for parsing.
#[derive(Debug)]
pub enum ParseContext<'source> {
    /// Parse a single element from a string
    Element(Hashed<&'source str>),
    /// Parse a source code snippet from a url and a string.
    Source {
        /// The source url
        url: Url,
        /// Line index
        line_index: LineIndex,
        /// A line offset (e.g. used when source is parsed from markdown code snippets).
        line_offset: u32,
        /// The source code to be parsed.
        code: Hashed<&'source str>,
    },
}

impl<'source> ParseContext<'source> {
    /// Create a new parse context for a source code.
    pub fn new(source: &'source str) -> Self {
        Self::Element(Hashed::new(source))
    }

    /// Add a URL to the parse context.
    pub fn with_url(self, url: Url) -> Self {
        match self {
            Self::Source {
                line_index,
                line_offset,
                code,
                ..
            } => Self::Source {
                url,
                code,
                line_index,
                line_offset,
            },
            Self::Element(code) => Self::Source {
                line_index: LineIndex::new(&code),
                url,
                code,
                line_offset: 0,
            },
        }
    }

    /// Add a line offset the parse context.
    pub fn with_line_offset(self, line_offset: u32) -> Self {
        match self {
            Self::Source {
                url,
                line_index,
                code: source,
                ..
            } => Self::Source {
                url,
                code: source,
                line_index,
                line_offset,
            },
            Self::Element(source) => Self::Source {
                line_index: LineIndex::new(&source),
                url: microcad_lang_base::virtual_url(&format!("source_{}", source.computed_hash())),
                code: source,
                line_offset,
            },
        }
    }

    /// Create a source code reference from a span.
    pub fn src_ref(&self, span: &Span) -> SrcRef {
        match self {
            Self::Source {
                line_index,
                line_offset,
                code: source,
                ..
            } => line_index
                .src_ref(source.value(), span, source.computed_hash())
                .with_line_offset(*line_offset),
            Self::Element(source) => {
                SrcRef::new(span.clone(), Default::default(), source.computed_hash())
            }
        }
    }
}

impl<'source> From<&'source microcad_lang_base::Source> for ParseContext<'source> {
    fn from(source: &'source microcad_lang_base::Source) -> Self {
        Self::Source {
            url: source.url.clone(),
            line_index: LineIndex::new(&source.code),
            line_offset: source.line_offset,
            code: Hashed::new(source.code.value()),
        }
    }
}
