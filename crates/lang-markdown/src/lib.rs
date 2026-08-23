// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language markdown support library.

mod block;
mod code_block;
mod markdown;
mod mdbook;
mod parser;
mod section;

pub use block::Block;
pub use code_block::{CodeBlock, CodeBlockHeader};
pub use markdown::{Markdown, MarkdownError};
pub use mdbook::{MdBook, MdBookError};
pub use section::Section;

pub use parser::ParseError;
pub use parser::parse;

/// Helper macro to construct a Markdown from any format string.
#[macro_export]
macro_rules! md {
    ($($arg:tt)*) => {
        $crate::parse(&format!($($arg)*)).unwrap_or_default()
    };
}

pub use microcad_lang_base::WriteToFile;
