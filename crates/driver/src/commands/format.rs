// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Command to format a document.

use microcad_lang_base::{LineCol, SrcRef};

use crate::Result;

/// Format parameters
pub type FormatParameters = microcad_lang_format::FormatConfig;

pub struct TextEdit {
    pub src_ref: SrcRef,
    pub new_text: String,
}

/// Function to turn the comparison of two `&str` in a `Vec<lsp::TextEdit>` as `dissimilar`
pub fn compare_strs_to_lsp_edits(old_str: &str, new_str: &str) -> Vec<TextEdit> {
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
                let start = LineCol::new(current_line, current_char);

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
                edits.push(TextEdit {
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

/// Format a document.
pub trait Format {
    fn format(&mut self, params: &FormatParameters) -> Result<Vec<TextEdit>>;
}
