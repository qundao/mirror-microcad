// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    ComputedHash, GetSourceByHash, HashId, Hashed, LineCol, LineIndex, SpanToSrcRef, SrcRef,
};
use serde::Serialize;

mod location;

pub use location::{SourceKind, SourceLocation};

/// Unparsed source code with a location.
///
/// A [`Source`] is a textual input for the compiler.
//////
/// Additionally, a unique hash of the source code computed.
#[derive(Debug, Clone, Serialize)]
pub struct Source {
    /// The source location
    pub location: SourceLocation,
    /// The original hashed code
    pub code: Hashed<String>,
    /// A line index to get lines from spans  
    line_index: LineIndex,
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
    pub fn new(location: impl Into<SourceLocation>, code: String) -> Self {
        let location = location.into();
        let line_index = LineIndex::new(&code, location.line_offset.unwrap_or_default());

        Self {
            location,
            code: Hashed::new(code),
            line_index,
        }
    }

    pub fn hash(&self) -> HashId {
        self.code.computed_hash()
    }

    pub fn code(&self) -> &str {
        self.code.value()
    }

    pub fn set_code(&mut self, code: String) {
        self.code = Hashed::new(code);
    }

    pub fn path(&self) -> Option<std::path::PathBuf> {
        self.location.path()
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
                    let start = current;
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

#[cfg(feature = "io")]
impl Source {
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        let path = path.as_ref().to_path_buf();
        let code = std::fs::read_to_string(&path)?;
        Ok(Self::new(SourceLocation::new(path), code))
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        std::fs::write(path, self.code().as_bytes())
    }
}

impl<'a> From<&'a str> for Source {
    fn from(s: &'a str) -> Self {
        Source::new(SourceKind::Str, s.to_string())
    }
}

impl<'a> SpanToSrcRef for &'a Source {
    fn span_to_src_ref(&self, span: &crate::Span) -> SrcRef {
        self.line_index.src_ref(self.code.as_str(), span)
    }
}

impl<'a> GetSourceByHash for &'a Source {
    fn get_source_by_hash(&self, hash: HashId) -> Option<&'a Source> {
        if hash == self.hash() {
            Some(self)
        } else {
            None
        }
    }
}
