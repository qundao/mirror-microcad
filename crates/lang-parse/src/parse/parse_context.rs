// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse context for parsing specific types.

use microcad_lang_base::{LineIndex, Source, Span, SpanToSrcRef, SrcRef};

/// Context for parsing.
#[derive(Debug)]
pub struct ParseContext<'source> {
    /// The source
    pub source: &'source Source,
    /// Line index
    pub line_index: LineIndex,
}

impl<'source> From<&'source Source> for ParseContext<'source> {
    fn from(source: &'source Source) -> Self {
        Self {
            source,
            line_index: LineIndex::from(source),
        }
    }
}

impl<'source> SpanToSrcRef for ParseContext<'source> {
    fn span_to_src_ref(&self, span: &Span) -> SrcRef {
        self.line_index.src_ref(self.source.code.as_str(), span)
    }
}
