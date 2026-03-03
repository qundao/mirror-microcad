// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a single markdown file for symbol.

mod markdown;
mod section;
mod to_md;

pub use markdown::Markdown;
use microcad_builtin::Symbol;
pub use section::Section;
pub use to_md::ToMd;

use crate::DocGen;

/// Markdown generator.
pub struct Md {
    pub _output_file: Option<std::path::PathBuf>,
}

impl DocGen for Md {
    fn doc_gen(&self, _symbol: &Symbol) -> std::io::Result<()> {
        todo!()
    }
}
