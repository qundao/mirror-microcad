// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax definitions and parser for µcad source code.
//!
//! This module includes the components to parse µcad source code into a stream of tokens or abstract syntax tree.
//!
//! - Transform source code into a stream of tokens with [`lex`]
//! - Create an abstract syntax tree from the list of tokens with [`parse`]

use std::ops::Range;

/// Span for tokens or AST nodes, a range of byte offsets from the start of the source
pub type Span = Range<usize>;

/// Abstract syntax tree for µcad files
pub mod ast;
mod parser;
/// Source tokens for µcad files
pub mod tokens;

pub use tokens::lex;
pub use parser::{parse, ParseError};
