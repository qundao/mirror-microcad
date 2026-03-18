// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language base components for error handling etc.

use miette::{MietteError, MietteSpanContents, SourceCode, SourceSpan, SpanContents};

mod diag;
mod rc;
mod src_ref;
mod tree_display;

/// Id type (base of all identifiers)
pub type Id = compact_str::CompactString;

/// List of valid µcad extensions.
pub const MICROCAD_EXTENSIONS: &[&str] = &["µcad", "mcad", "ucad"];

pub use diag::{
    Diag, DiagError, DiagHandler, DiagRenderOptions, DiagResult, Diagnostic, Level, PushDiag,
};
pub use rc::{Rc, RcMut};
pub use src_ref::{Refer, SrcRef, SrcRefInner, SrcReferrer};
pub use tree_display::{FormatTree, TreeDisplay, TreeState};

/// A compatibility layer for using SourceFile with miette
pub struct MietteSourceFile<'a> {
    source: &'a str,
    name: String,
    line_offset: usize,
}

impl MietteSourceFile<'static> {
    /// Create an invalid source file for when we can't load the source
    pub fn invalid() -> Self {
        MietteSourceFile {
            source: crate::invalid_no_ansi!(FILE),
            name: crate::invalid_no_ansi!(FILE).into(),
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

#[cfg(not(feature = "ansi-color"))]
#[macro_export]
macro_rules! found {
    (FOUND) => {
        "Found"
    };
    (FINAL) => {
        "Found"
    };
    (INTERMEDIATE) => {
        "Found"
    };
    (MATCH) => {
        "Match"
    };
    (NO_MATCH) => {
        "No Match"
    };
    (CALL) => {
        "Call"
    };
    (LOOKUP) => {
        "Lookup"
    };
    (LOAD) => {
        "Loading"
    };
    (RESOLVE) => {
        "Resolve"
    };
    (AMBIGUOUS) => {
        "Ambiguous"
    };
    (NOT_FOUND) => {
        "Not found"
    };
    (NOT_FOUND) => {
        "Not found"
    };
}

/// Generate string literal ` INVALID `*XXX*` ` with ANSI color.
#[cfg(feature = "ansi-color")]
#[macro_export]
macro_rules! invalid {
    (VALUE) => {
        color_print::cstr!("<R!,k,s> NO VALUE </>")
    };
    (TYPE) => {
        color_print::cstr!("<R!,k,s> NO TYPE </>")
    };
    (OUTPUT) => {
        color_print::cstr!("<R!,k,s> NO OUTPUT </>")
    };
    (STACK) => {
        color_print::cstr!("<W,k,s> EMPTY STACK </>")
    };
    (REF) => {
        color_print::cstr!("<Y!,k,s> NO REF </>")
    };
    (FILE) => {
        color_print::cstr!("<Y!,k,s> NO FILE </>")
    };
    (RESULT) => {
        color_print::cstr!("<Y!,k,s> NO RESULT </>")
    };
    (LINE) => {
        color_print::cstr!("<Y!,k,s> NO LINE </>")
    };
    (SOURCE) => {
        color_print::cstr!("<C!,k,s> FROM STR </>")
    };
    (UNKNOWN) => {
        color_print::cstr!("<M!,k,s> UNKNOWN </>")
    };
    (ID) => {
        color_print::cstr!("<M!,k,s> NO ID </>")
    };
    (NAME) => {
        color_print::cstr!("<M!,k,s> NO NAME </>")
    };
    (EXPRESSION) => {
        color_print::cstr!("<R!,k,s> INVALID EXPRESSION </>")
    };
}

/// Generate string literal `<INVALID `*XXX*`>`.
#[macro_export]
macro_rules! invalid_no_ansi {
    (VALUE) => {
        "<NO VALUE>"
    };
    (TYPE) => {
        "<NO TYPE>"
    };
    (OUTPUT) => {
        "<NO OUTPUT>"
    };
    (STACK) => {
        "<EMPTY STACK>"
    };
    (REF) => {
        "<NO REF>"
    };
    (FILE) => {
        "<NO FILE>"
    };
    (RESULT) => {
        "<NO RESULT>"
    };
    (LINE) => {
        "<NO LINE>"
    };
    (SOURCE) => {
        "<FROM STR>"
    };
    (UNKNOWN) => {
        "<UNKNOWN>"
    };
    (ID) => {
        "<NO ID>"
    };
    (NAME) => {
        "<INVALID NAME>"
    };
    (EXPRESSION) => {
        "<INVALID EXPRESSION>"
    };
}

#[macro_export]
#[cfg(not(feature = "ansi-color"))]
macro_rules! invalid {
    ($x:literal) => {
        invalid_no_ansi!($x)
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
