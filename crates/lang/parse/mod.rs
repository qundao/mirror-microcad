// Copyright © 2024-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source code parsing
//!
//! A source file on disc is just a bunch of UTF-8 encoded text which must be parsed
//! before any processing:
//!
//! ```no_run
//! use microcad_lang::{syntax::*, parse::*};
//!
//! let source_file = SourceFile::load("my.µcad").expect("parsing success");
//! ```
//!
//! To read a source file from an already loaded string use:
//!
//! ```no_run
//! use microcad_lang::{syntax::*, parse::*};
//!
//! let source_file = SourceFile::load_from_str(Some("test"), "test.µcad", r#"std::print("hello world!");"#).expect("parsing success");
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be resolved and evaluated (see [`crate::resolve`] and [`crate::eval`])  .

mod attribute;
mod body;
mod call;
mod doc_block;
mod expression;
mod format_string;
mod function;
mod identifier;
mod init_definition;
mod lang_type;
mod literal;
mod module;
mod parameter;
mod source_file;
mod statement;
mod r#type;
mod r#use;
mod workbench;

pub(crate) mod parse_error;

use microcad_syntax::{lex, parse};
pub use parse_error::*;

use crate::{src_ref::*, syntax::*};
use crate::parser::ParseContext;

pub(crate) fn build_ast(source: &str, parse_context: &ParseContext) -> Result<microcad_syntax::ast::SourceFile, ParseErrorsWithSource> {
    let tokens: Vec<_> = lex(source).collect();
    parse(tokens.as_slice()).map_err(|errors| {
        let errors = errors
            .into_iter()
            .map(|error| {
                let src_ref = parse_context.src_ref(&error.span);
                ParseError::AstParser(Refer::new(error, src_ref))
            })
            .collect::<Vec<_>>();
        ParseErrorsWithSource {
            errors,
            source_code: Some(source.into()),
            source_hash: parse_context.source_file_hash,
        }
    })
}
