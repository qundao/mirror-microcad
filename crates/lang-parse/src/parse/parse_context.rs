// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse context for parsing specific types.

use microcad_lang_base::{Diagnostic, Diagnostics, LineIndex, Source, Span, SpanToSrcRef, SrcRef};

use crate::ParseErrors;

/// Context for parsing.
#[derive(Debug)]
pub struct ParseContext<'source> {
    /// The source
    pub source: &'source Source,
    /// Line index
    pub line_index: LineIndex,
}

impl<'source> ParseContext<'source> {
    /// Return diagnostics for parser errors
    pub fn diagnostics(&self, errors: ParseErrors) -> Diagnostics {
        use microcad_lang_base::SpanToSrcRef;
        let mut diag_list = Diagnostics::default();
        for err in errors.0 {
            diag_list.push(Diagnostic {
                src_ref: self.span_to_src_ref(&err.span),
                report: err.into(),
            })
        }

        diag_list
    }
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
