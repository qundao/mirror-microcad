// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax definitions and parser for µcad source code.
//!
//! This module includes the components to parse µcad source code into a stream of tokens or abstract syntax tree.
//!
//! - Transform source code into a stream of tokens with [`lex`]
//! - Create an abstract syntax tree from the list of tokens with [`parse`]

/// Abstract syntax tree for µcad files
pub mod ast;
mod parser;

mod source;

/// Source tokens for µcad files
pub mod tokens;

pub use parser::ParseError;

/// Highlevel API to parse directly from a string
pub fn parse(source: &str) -> Result<ast::Program, Vec<ParseError>> {
    parser::parse(&tokens::lex(source).collect::<Vec<_>>())
}

pub use source::Source;
