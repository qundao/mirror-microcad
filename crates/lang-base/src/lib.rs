// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language base components for error handling etc.

use std::str::FromStr;

use miette::{MietteError, MietteSpanContents, SourceCode, SourceSpan, SpanContents};

mod code_display;
mod diag;
mod identifier;
mod ord_map;
mod output;
mod rc;
mod src_ref;
mod tree_display;

/// Id type (base of all identifiers)
pub type Id = compact_str::CompactString;

/// URL to locate sources.
pub use url::Url;

pub fn virtual_url() -> Url {
    Url::from_str("virtual://file").unwrap()
}

/// List of valid µcad extensions.
pub const MICROCAD_EXTENSIONS: &[&str] = &["µcad", "mcad", "ucad"];

pub use code_display::*;
pub use diag::{
    Diag, DiagError, DiagHandler, DiagRenderOptions, DiagResult, Diagnostic, Diagnostics, Level,
    PushDiag,
};
pub use identifier::Identifier;
pub use ord_map::{OrdMap, OrdMapValue};
pub use output::{Capture, Output, Stdout};
pub use rc::{Rc, RcMut};
pub use src_ref::{Refer, SrcRef, SrcRefInner, SrcReferrer};
pub use tree_display::{FormatTree, TreeDisplay, TreeState};

pub use microcad_core::hash::{ComputedHash, HashId, HashMap, HashSet, Hashed};

/// A compatibility layer for using SourceFile with miette
pub struct MietteSourceFile<'a> {
    /// The source text.
    pub source: &'a str,
    /// Name of of file
    pub name: String,
    /// Line offset (e.g. used when source comes from a markdown file).
    pub line_offset: usize,
}

impl MietteSourceFile<'static> {
    /// Create an invalid source file for when we can't load the source
    pub fn invalid() -> Self {
        MietteSourceFile {
            source: "NO FILE",
            name: "NO FILE".into(),
            line_offset: 0,
        }
    }
}

impl SourceCode for MietteSourceFile<'_> {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn SpanContents<'a> + 'a>, MietteError> {
        let inner_contents =
            self.source
                .read_span(span, context_lines_before, context_lines_after)?;
        let contents = MietteSpanContents::new_named(
            self.name.clone(),
            inner_contents.data(),
            *inner_contents.span(),
            inner_contents.line() + self.line_offset,
            inner_contents.column(),
            inner_contents.line_count(),
        )
        .with_language("µcad");
        Ok(Box::new(contents))
    }
}

/// Trait that can fetch for a file by it's hash value.
pub trait GetSourceStrByHash {
    /// Get a source string by it's hash value.
    fn get_str_by_hash(&self, hash: u64) -> Option<&str>;

    /// Get filename by hash
    fn get_filename_by_hash(&self, hash: u64) -> Option<std::path::PathBuf>;
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

/// Shortens given string to it's first line and to maximum characters.
#[macro_export]
macro_rules! shorten {
    ($what:expr) => {
        $crate::shorten(&format!("{}", $what), 140)
    };
    ($what:expr,$shorten:expr) => {
        if $shorten {
            $crate::shorten!($what)
        } else {
            $what
        }
    };
    ($what:expr, $max_chars:literal) => {
        shorten(format!("{}", $what).lines(), max_chars)
    };
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
