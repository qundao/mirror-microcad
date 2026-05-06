// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source API

use microcad_lang_base::{
    ComputedHash, Diagnostic, Diagnostics, GetSourceStrByHash, Hashed, LineIndex, PushDiag, Refer,
    SrcRef, SrcReferrer, Url,
};

use crate::ast;

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
        self.line_index.span_to_src_ref(
            &self.text,
            self.ast.span.clone(),
            self.text.computed_hash(),
        )
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
