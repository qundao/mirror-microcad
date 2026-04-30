// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language markdown support library.

mod code_block;
mod markdown;
mod mdbook;
mod paragraph;
mod parser;
mod section;

pub use code_block::CodeBlock;
pub use markdown::{Markdown, MarkdownError};
pub use mdbook::{MdBook, MdBookError};
pub use paragraph::Paragraph;
pub use section::Section;

pub use parser::ParseError;
pub use parser::parse;
