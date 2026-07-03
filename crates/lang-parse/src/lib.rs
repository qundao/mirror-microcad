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
use microcad_lang_base::{CompilationResult, Diagnostics, Source};
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
        parse::parse(&lex(context.source.code()).collect::<Vec<_>>())
    }
}

pub use lex::lex;

/// Parse a source into an abstract syntax tree.
pub fn parse(source: &Source) -> CompilationResult<Ast> {
    let context = ParseContext::from(source);
    match Ast::parse(&context) {
        Ok(ast) => Ok((ast, Diagnostics::default())), // FIXME: Right now, the parser can only return errors and no warnings
        Err(errors) => Err(context.diagnostics(errors)),
    }
}
