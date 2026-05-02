// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source API

use microcad_lang_base::{
    ComputedHash, Diagnostic, Diagnostics, GetSourceStrByHash, HashId, Hashed, PushDiag, Refer,
    SrcRef, SrcReferrer, Url,
};

use crate::ast;

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

    fn span_to_src_ref(&self, text: &str, span: ast::Span, hash: HashId) -> SrcRef {
        let (line, col) = self.line_col(text, span.start);
        SrcRef::new(span.clone(), line, col, hash)
    }
}

/// A µcad source with a parse syntax tree with a line index and the hashed original source code.
pub struct Source {
    /// The source url
    pub url: Url,
    /// The original text
    pub text: Hashed<String>,
    /// The µcad program
    pub ast: ast::Program,

    /// Computed line index.
    line_index: LineIndex,
}

impl Source {
    /// When you have a location
    pub fn new(url: Url, content: String) -> Result<Self, Diagnostics> {
        let line_index = LineIndex::new(&content);
        let text = Hashed::new(content.to_string());

        Ok(Self {
            url,
            ast: crate::parse(&content).map_err(|errors| {
                let mut diag_list = Diagnostics::default();

                for err in errors {
                    let span = err.span.clone();
                    diag_list
                        .push_diag(Diagnostic::Error(Refer::new(
                            err.into(),
                            line_index.span_to_src_ref(text.as_str(), span, text.computed_hash()),
                        )))
                        .expect("Diag list should return no error");
                }

                diag_list
            })?,
            line_index,
            text,
        })
    }

    /// When you just have a string (e.g. tests or REPL)
    pub fn from_string(content: String) -> Result<Self, Diagnostics> {
        Self::new(microcad_lang_base::virtual_url(), content)
    }
}

impl SrcReferrer for Source {
    fn src_ref(&self) -> SrcRef {
        let (line, col) = self.line_index.line_col(&self.text, self.ast.span.start);
        SrcRef::new(self.ast.span.clone(), line, col, self.text.computed_hash())
    }
}

impl GetSourceStrByHash for Source {
    fn get_str_by_hash(&self, hash: u64) -> Option<&str> {
        if hash == self.text.computed_hash() {
            Some(self.text.as_str())
        } else {
            None
        }
    }

    fn get_filename_by_hash(&self, hash: u64) -> Option<std::path::PathBuf> {
        if hash == self.text.computed_hash() {
            self.url.to_file_path().ok()
        } else {
            None
        }
    }
}
