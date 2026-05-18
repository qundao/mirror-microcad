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

/// Source tokens for µcad files
pub mod tokens;

pub use parser::{ParseContext, ParseError, ParseErrors, parsers};

/// Parse trait.
pub trait Parse: Sized {
    /// Parse from a context.
    ///
    /// The context also contains the source string.
    fn parse(context: &ParseContext) -> Result<Self, ParseErrors>;
}

impl Parse for ast::Source {
    fn parse(context: &ParseContext) -> Result<Self, ParseErrors> {
        match context {
            ParseContext::Element(_) => panic!("Expected parse source context"),
            ParseContext::Source {
                url,
                line_offset,
                code,
                ..
            } => {
                let ast = crate::parse(code.value())?;
                let src_ref = context.src_ref(&ast.span);

                Ok(Self {
                    url: url.clone(),
                    ast: microcad_lang_base::Refer::new(ast, src_ref),
                    line_offset: *line_offset,
                    code: code.clone().map(|s| s.to_string()),
                })
            }
        }
    }
}

/// API to parse directly from a string
pub fn parse(source: &str) -> Result<ast::Program, ParseErrors> {
    parser::parse(&tokens::lex(source).collect::<Vec<_>>())
}
