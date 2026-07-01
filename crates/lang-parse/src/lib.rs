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

/// Tokens
pub mod token;

/// Contains the parser.
mod parse;

/// Contains the lexer (aka tokenizer).
mod lex;

pub use ast::Ast;
pub use parse::{ParseContext, ParseError, ParseErrors, parsers};

/// Parse trait.
pub trait Parse: Sized {
    /// Parse from a context.
    ///
    /// The context also contains the source string.
    fn parse(context: &ParseContext) -> Result<Self, ParseErrors>;
}

impl Parse for Ast {
    fn parse(context: &ParseContext) -> Result<Self, ParseErrors> {
        Ok(crate::parse(context.source.code())?)
    }
}

pub use lex::lex;

/// API to parse directly from a string
pub fn parse(source: &str) -> Result<Ast, ParseErrors> {
    parse::parse(&lex(source).collect::<Vec<_>>())
}
