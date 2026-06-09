// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Various type conversion from µcad to tower_lsp's types.

use tower_lsp::lsp_types as lsp;

use microcad_driver::prelude as mu;
use mu::traits::*;

pub trait ToLsp {
    type Output;

    fn to_lsp(&self) -> Self::Output;
}

impl ToLsp for mu::SrcRef {
    type Output = Option<lsp::Range>;

    fn to_lsp(&self) -> Self::Output {
        match self.is_some() {
            true => {
                let start = lsp::Position::new(self.at.line, self.at.col - 1);
                let end = lsp::Position::new(self.at.line, (self.at.col + self.len() as u32) - 1);

                Some(lsp::Range::new(start, end))
            }
            false => None,
        }
    }
}

impl ToLsp for mu::base::DiagLevel {
    type Output = lsp::DiagnosticSeverity;

    fn to_lsp(&self) -> Self::Output {
        use mu::base::DiagLevel::*;
        match &self {
            Trace => lsp::DiagnosticSeverity::HINT,
            Info => lsp::DiagnosticSeverity::INFORMATION,
            Warning => lsp::DiagnosticSeverity::WARNING,
            Error => lsp::DiagnosticSeverity::ERROR,
        }
    }
}

impl ToLsp for mu::Diagnostics {
    type Output = lsp::FullDocumentDiagnosticReport;

    fn to_lsp(&self) -> Self::Output {
        lsp::FullDocumentDiagnosticReport {
            result_id: None,
            items: self
                .iter()
                .filter_map(|diag| {
                    let message = diag.message();
                    diag.src_ref().to_lsp().map(|range| {
                        lsp::Diagnostic::new(
                            range,
                            Some(diag.level().to_lsp()),
                            None,
                            None,
                            message,
                            None,
                            None,
                        )
                    })
                })
                .collect(),
        }
    }
}

/// Function to turn the comparison of two `&str` in a `Vec<lsp::TextEdit>` as `dissimilar`
pub fn compare_strs_to_lsp_edits(old_str: &str, new_str: &str) -> Vec<lsp::TextEdit> {
    use dissimilar::Chunk;
    let chunks = dissimilar::diff(old_str, new_str);
    let mut edits = Vec::new();

    // Track the current position in the *old_str*
    let mut current_line = 0;
    let mut current_char = 0;

    for chunk in chunks {
        match chunk {
            Chunk::Equal(text) => {
                // Just move the cursor forward based on the matching text
                for ch in text.chars() {
                    if ch == '\n' {
                        current_line += 1;
                        current_char = 0;
                    } else {
                        current_char += 1;
                    }
                }
            }
            Chunk::Delete(text) => {
                // Define the start position of the deletion
                let start = lsp::Position::new(current_line, current_char);

                // Calculate the end position by walking through the deleted text
                for ch in text.chars() {
                    if ch == '\n' {
                        current_line += 1;
                        current_char = 0;
                    } else {
                        current_char += 1;
                    }
                }
                let end = lsp::Position::new(current_line, current_char);

                // A deletion replaces the range with an empty string
                edits.push(lsp::TextEdit {
                    range: lsp::Range::new(start, end),
                    new_text: String::new(),
                });
            }
            Chunk::Insert(text) => {
                // An insertion happens at the *current* position without moving the cursor forward
                // (since the inserted text doesn't exist in the original document)
                let pos = lsp::Position::new(current_line, current_char);

                edits.push(lsp::TextEdit {
                    range: lsp::Range::new(pos, pos),
                    new_text: text.to_string(),
                });
            }
        }
    }

    edits
}
