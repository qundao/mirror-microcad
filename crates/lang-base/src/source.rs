// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    GetSourceLocInfoByHash, HashId, Hashed, LineCol, ResourceLocation, SourceLocInfo, SrcRef,
};
use microcad_core::hash::ComputedHash;
use serde::Serialize;
use url::Url;

/// Unparsed source code with a location.
///
/// The unique location of µcad file is specified via:
/// * `url: Url`: A URL pointing to a source code
/// * `line_offset: u32`: A line offset inside file
///
/// Additionally, a unique hash of the source code computed.
#[derive(Debug, Clone, Serialize)]
pub struct Source {
    /// The source url
    pub url: Url,
    /// Line offset
    pub line_offset: u32,
    /// The original hashed code
    pub code: Hashed<String>,
}

/// A text edit, the result of comparing two sources
#[derive(Debug, PartialEq)]
pub struct TextEdit {
    /// SrcRef of a TextEdit
    pub src_ref: SrcRef,
    /// New text.
    pub new_text: String,
}

impl Source {
    /// Create a new source.
    pub fn new(url: Url, line_offset: u32, code: String) -> Self {
        Self {
            url,
            line_offset,
            code: Hashed::new(code),
        }
    }

    pub fn code(&self) -> &str {
        self.code.value()
    }

    pub fn set_code(&mut self, code: String) {
        self.code = Hashed::new(code);
    }

    /// Compare two sources and return a vector of TextEdits.
    pub fn compare(&self, other: &Self) -> Vec<TextEdit> {
        use dissimilar::Chunk;
        let chunks = dissimilar::diff(&self.code, &other.code);
        let mut edits = Vec::new();
        let source_hash = self.code.computed_hash();

        // Track the current position in the *old_str*
        let mut current = LineCol::default();
        let mut byte_offset = 0;

        for chunk in chunks {
            match chunk {
                Chunk::Equal(text) => {
                    // Just move the cursor forward based on the matching text
                    for ch in text.chars() {
                        byte_offset += ch.len_utf8();

                        if ch == '\n' {
                            current.line += 1;
                            current.col = 0;
                        } else {
                            current.col += 1;
                        }
                    }
                }
                Chunk::Delete(text) => {
                    // Define the start position of the deletion
                    let start = current.clone();
                    let start_byte = byte_offset;

                    // Calculate the end position by walking through the deleted text
                    for ch in text.chars() {
                        byte_offset += ch.len_utf8();

                        if ch == '\n' {
                            current.line += 1;
                            current.col = 0;
                        } else {
                            current.col += 1;
                        }
                    }

                    // A deletion replaces the range with an empty string
                    edits.push(TextEdit {
                        src_ref: SrcRef::new(&(start_byte..byte_offset), start, source_hash),
                        new_text: String::new(),
                    });
                }
                Chunk::Insert(text) => {
                    // An insertion happens at the *current* position without moving the cursor forward
                    // (since the inserted text doesn't exist in the original document)
                    edits.push(TextEdit {
                        src_ref: SrcRef::new(&(byte_offset..byte_offset), current, source_hash),
                        new_text: text.to_string(),
                    });
                }
            }
        }

        edits
    }
}

impl GetSourceLocInfoByHash for Source {
    fn get_source_loc_info_by_hash(&'_ self, hash: HashId) -> Option<SourceLocInfo<'_>> {
        if hash == self.code.computed_hash() {
            Some(SourceLocInfo {
                code: &self.code,
                url: self.url.clone(),
                line_offset: self.line_offset,
            })
        } else {
            None
        }
    }
}

impl ResourceLocation for Source {
    fn url(&self) -> &Url {
        &self.url
    }
}
