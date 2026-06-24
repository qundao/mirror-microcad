// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{GetSourceLocInfoByHash, HashId, Hashed, LineCol, SourceLocInfo, SrcRef};
use microcad_core::hash::ComputedHash;
use serde::Serialize;
use url::Url;

/// Kind of the source file
#[derive(Debug, Clone, Serialize, PartialEq, Eq, derive_more::From)]
pub enum SourceKind {
    Url(Url),
    Path(std::path::PathBuf),
    Stdin,
    Virtual(String),
}

impl SourceKind {
    /// Returns a fallback or explicit URL representation of the source.
    pub fn url(&self) -> Url {
        match self {
            SourceKind::Url(url) => url.clone(),
            SourceKind::Path(path) => {
                // Safely convert a local path to a file:// URL
                Url::from_file_path(path)
                    .unwrap_or_else(|_| Url::parse("file:///invalid-path").unwrap())
            }
            SourceKind::Stdin => Url::parse("stdin://-").unwrap(),
            SourceKind::Virtual(name) => {
                // URL-encode or format the virtual name into a schema
                let scheme = format!("virtual://{}", name);
                Url::parse(&scheme).unwrap_or_else(|_| Url::parse("virtual://unknown").unwrap())
            }
        }
    }

    /// Returns a reference to the underlying path if this source lives on disk.
    pub fn path(&self) -> Option<std::path::PathBuf> {
        match self {
            SourceKind::Path(path) => Some(path.clone()),
            SourceKind::Url(url) => {
                // If it's a file:// URL, we can extract the path dynamically
                if url.scheme() == "file" {
                    url.to_file_path().ok()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Return the relative file path from current directory.
    pub fn relative_path(&self) -> Option<std::path::PathBuf> {
        self.path().map(|path| {
            let current_dir = std::env::current_dir().expect("current dir");
            if let Ok(path) = path.canonicalize() {
                pathdiff::diff_paths(path, current_dir).unwrap_or_default()
            } else {
                path.to_path_buf()
            }
        })
    }

    /// Helper to identify if the resource exists on disk.
    pub fn is_local(&self) -> bool {
        self.path().is_some()
    }

    /// The source name
    pub fn source_name(&self) -> String {
        self.relative_path()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or(self.url().path().to_string())
    }
}

/// Represents *where* the code came from.
/// Stored once in the central SourceMap, indexed by SourceId.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SourceLocation {
    pub kind: SourceKind,
    pub line_offset: Option<u32>,
}

impl SourceLocation {
    /// New source location with specific source kind
    pub fn new(kind: impl Into<SourceKind>) -> Self {
        Self {
            kind: kind.into(),
            line_offset: None,
        }
    }

    /// New source location with a line offset
    pub fn with_line_offset(self, line_offset: u32) -> Self {
        Self {
            kind: self.kind,
            line_offset: Some(line_offset),
        }
    }

    /// Forwards to the underlying `SourceKind::url`
    #[inline]
    pub fn url(&self) -> Url {
        self.kind.url()
    }

    /// Forwards to the underlying `SourceKind::path`
    #[inline]
    pub fn path(&self) -> Option<std::path::PathBuf> {
        self.kind.path()
    }
}

impl From<SourceKind> for SourceLocation {
    fn from(kind: SourceKind) -> Self {
        Self::new(kind)
    }
}

/// Unparsed source code with a location.
///
/// A [`Source`] is a textual input for the compiler.
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
