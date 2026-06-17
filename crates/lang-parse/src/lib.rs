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
mod parse;

/// Source tokens for µcad files
pub mod lex;

use microcad_lang_base::virtual_url;
pub use parse::{ParseContext, ParseError, ParseErrors, parsers};

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
            ParseContext::Element(code) => {
                let ast = crate::parse(code)?;
                let src_ref = context.src_ref(&ast.span);

                Ok(Self {
                    url: virtual_url("virtual"),
                    ast: microcad_lang_base::Refer::new(ast, src_ref),
                    line_offset: 0,
                    code: code.clone().map(|s| s.to_string()),
                })
            }
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
    parse::parse(&lex::lex(source).collect::<Vec<_>>())
}
