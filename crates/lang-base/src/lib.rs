// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language base components for error handling etc.

use std::str::FromStr;

use miette::{MietteError, MietteSpanContents, SourceCode, SourceSpan, SpanContents};

mod code_display;
mod diag;
pub mod element;
mod ord_map;
mod output;
mod rc;
mod source;
mod src_ref;
mod tree_display;

pub use compact_str::{CompactString, ToCompactString};

/// Id type (base of all identifiers)
pub type Id = CompactString;

/// URL to locate sources.
pub use url::Url;

pub fn virtual_url(name: &str) -> Url {
    Url::from_str(&format!("virtual://{name}")).unwrap()
}

pub trait ResourceLocation {
    /// The canonical identity of the resource.
    fn url(&self) -> &Url;

    /// Attempts to convert the location to a physical filesystem path.
    /// Returns None if the resource is virtual (e.g., ucad-std:// or snippet://).
    fn to_file_path(&self) -> Option<std::path::PathBuf> {
        if self.url().scheme() == "file" {
            self.url().to_file_path().ok()
        } else {
            None
        }
    }

    /// Helper to identify if the resource exists on disk.
    fn is_local(&self) -> bool {
        self.url().scheme() == "file"
    }

    /// Return the relative file path from current directory.
    fn relative_path(&self) -> Option<std::path::PathBuf> {
        self.to_file_path().map(|path| {
            let current_dir = std::env::current_dir().expect("current dir");
            if let Ok(path) = path.canonicalize() {
                pathdiff::diff_paths(path, current_dir).unwrap_or_default()
            } else {
                path.to_path_buf()
            }
        })
    }

    /// The source name
    fn source_name(&self) -> String {
        self.relative_path()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or(self.url().path().to_string())
    }
}

/// List of valid µcad extensions.
pub const MICROCAD_EXTENSIONS: &[&str] = &["mu", "µcad", "mcad", "ucad"];

/// Default extension for µcad files.
pub const MICROCAD_EXTENSION: &str = "µcad";

pub use code_display::*;
pub use diag::{
    Diag, DiagError, DiagHandler, DiagRenderOptions, DiagResult, Diagnostic, Diagnostics, Level,
    Level as DiagLevel, PushDiag,
};
pub use element::{Identifier, IdentifierList};
pub use ord_map::{OrdMap, OrdMapValue};
pub use output::{Capture, Output, Stdout};
pub use rc::{Rc, RcMut};
pub use src_ref::{LineCol, LineIndex, Refer, Span, Spanned, SrcRef, SrcReferrer};
pub use tree_display::{FormatTree, TreeDisplay, TreeState};

pub use microcad_core::hash::{ComputedHash, HashId, HashMap, HashSet, Hashed, Hasher};
pub use source::Source;

/// A compatibility layer for using SourceFile with miette
pub struct SourceLocInfo<'a> {
    /// The source text.
    pub code: &'a str,
    /// Name of of file
    pub url: Url,
    /// Line offset (e.g. used when source comes from a markdown file).
    pub line_offset: u32,
}

impl SourceLocInfo<'static> {
    /// Create an invalid source file for when we can't load the source
    pub fn invalid() -> Self {
        SourceLocInfo {
            code: "NO FILE",
            url: virtual_url("invalid"),
            line_offset: 0,
        }
    }
}

impl<'a> ResourceLocation for SourceLocInfo<'a> {
    fn url(&self) -> &Url {
        &self.url
    }
}

impl SourceCode for SourceLocInfo<'_> {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn SpanContents<'a> + 'a>, MietteError> {
        let inner_contents =
            self.code
                .read_span(span, context_lines_before, context_lines_after)?;
        let contents = MietteSpanContents::new_named(
            self.source_name(),
            inner_contents.data(),
            *inner_contents.span(),
            inner_contents.line() + self.line_offset as usize,
            inner_contents.column(),
            inner_contents.line_count(),
        )
        .with_language("µcad");
        Ok(Box::new(contents))
    }
}

/// Trait that can fetch for a file by it's hash value.
pub trait GetSourceLocInfoByHash {
    /// Get a source string by it's hash value.
    fn get_source_loc_info_by_hash(&'_ self, hash: HashId) -> Option<SourceLocInfo<'_>>;
}

/// Shortens given string to it's first line and to `max_chars` characters.
pub fn shorten(what: &str, max_chars: usize) -> String {
    let short: String = what
        .chars()
        .enumerate()
        .filter_map(|(p, ch)| {
            if p == max_chars {
                Some('…')
            } else if p < max_chars {
                if ch == '\n' { Some('⏎') } else { Some(ch) }
            } else {
                None
            }
        })
        .collect();

    if cfg!(feature = "ansi-color") && short.contains('\x1b') {
        short + "\x1b[0m"
    } else {
        short
    }
}

/// Create a marker string which is colored with ANSI.
#[cfg(feature = "ansi-color")]
#[macro_export]
macro_rules! mark {
    (FOUND!) => {
        color_print::cformat!("<G!,k,s> FOUND </>")
    };
    (FOUND) => {
        color_print::cformat!("<W!,k,s> FOUND </>")
    };
    (MATCH) => {
        color_print::cformat!("<Y!,k,s> MATCH </>")
    };
    (NO_MATCH) => {
        color_print::cformat!("<Y,k,s> NO MATCH </>")
    };
    (MATCH!) => {
        color_print::cformat!("<G!,k,s> MATCH </>")
    };
    (NO_MATCH!) => {
        color_print::cformat!("<R,k,s> NO MATCH </>")
    };
    (CALL) => {
        color_print::cformat!("<B,k,s> CALL </>")
    };
    (LOOKUP) => {
        color_print::cformat!("<c,s>LOOKUP</>")
    };
    (LOAD) => {
        color_print::cformat!("<Y,k,s> LOADING </>")
    };
    (RESOLVE) => {
        color_print::cformat!("<M,k,s> RESOLVE </>")
    };
    (AMBIGUOUS) => {
        color_print::cformat!("<R,k,s> AMBIGUOUS </>")
    };
    (NOT_FOUND!) => {
        color_print::cformat!("<R,k,s> NOT FOUND </>")
    };
    (NOT_FOUND) => {
        color_print::cformat!("<Y,k,s> NOT FOUND </>")
    };
}

/// Trait to write something with Display trait into a file.
pub trait WriteToFile: std::fmt::Display {
    /// Write something to a file.
    fn write_to_file(&self, filename: &impl AsRef<std::path::Path>) -> std::io::Result<()> {
        use std::io::Write;
        let file = std::fs::File::create(filename)?;
        let mut writer = std::io::BufWriter::new(file);
        write!(writer, "{self}")
    }
}
