// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language base components for error handling etc.

use std::str::FromStr;

use miette::{MietteError, MietteSpanContents, SourceCode, SourceSpan, SpanContents};

pub use miette::Severity;

mod artifact;
mod diag;
pub mod element;
mod ord_map;
mod output;
mod rc;
mod source;
mod src_ref;
mod tree_display;
mod version;

pub use compact_str::{CompactString, ToCompactString};

/// Id type (base of all identifiers)
pub type Id = CompactString;

/// URL to locate sources.
pub use url::Url;

pub fn virtual_url(name: &str) -> Url {
    Url::from_str(&format!("virtual://{name}")).unwrap()
}

/// List of valid µcad extensions.
pub const MICROCAD_EXTENSIONS: &[&str] = &["mu", "µcad", "mcad", "ucad"];

/// Default extension for µcad files.
pub const MICROCAD_EXTENSION: &str = "µcad";

pub use version::{MICROCAD_VERSION, Version};

pub use artifact::{Artifact, ArtifactKind};
pub use diag::{DiagRenderOptions, Diagnostic, Diagnostics};
pub use element::{Identifier, IdentifierList};
pub use ord_map::{OrdMap, OrdMapValue};
pub use output::{Capture, Output, Stdout};
pub use rc::{Rc, RcMut};
pub use src_ref::{LineCol, LineIndex, Refer, Span, SpanToSrcRef, Spanned, SrcRef, SrcReferrer};
pub use tree_display::{FormatTree, TreeDisplay, TreeState};

pub use microcad_core::hash::{ComputedHash, HashId, HashMap, HashSet, Hashed, Hasher};
pub use source::{Source, SourceKind, SourceLocation, Sourced, TextEdit};

impl SourceCode for Source {
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
            self.location.kind.source_name(),
            inner_contents.data(),
            *inner_contents.span(),
            inner_contents.line() + self.location.line_offset.unwrap_or_default() as usize,
            inner_contents.column(),
            inner_contents.line_count(),
        )
        .with_language("µcad");
        Ok(Box::new(contents))
    }
}

/// Trait that can fetch for a file by it's hash value.
pub trait GetSourceByHash {
    /// Get a source string by it's hash value.
    fn get_source_by_hash(&'_ self, hash: HashId) -> Option<&Source>;
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

/// A result that contains the compilation artifact bundled with diagnostics.
pub type CompilationResult<T> = Result<(T, Diagnostics), Diagnostics>;
