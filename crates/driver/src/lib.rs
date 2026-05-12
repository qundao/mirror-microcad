// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad driver is a high-level API to be integrated in LSP, CLI or Viewer.

pub mod commands;
mod config;
pub mod document;
mod session;
mod watcher;

use microcad_lang::lower::Lower;
use microcad_lang::value::Value;

pub use microcad_lang_parse::ast;

pub mod base {
    pub use microcad_lang_base::Source;
}

pub mod parse {
    pub use microcad_lang_parse::{Parse, ParseContext};
}

pub use microcad_lang::lower::ir;
pub mod lower {
    pub use microcad_lang::lower::LowerContext;
}

pub use microcad_lang::model::Model;
pub use microcad_lang::render::{RenderCache, RenderContext};
pub use microcad_lang_base::{RcMut, Url};

pub use config::Config;
pub use document::Document;
pub use session::Session;
pub use watcher::Watcher;

/// Parse a value from a string containing a literal.
pub fn value_from_str(s: &str) -> document::Result<Value> {
    let parse_context = parse::ParseContext::new(s);
    use parse::Parse;
    ir::Literal::lower(
        &ast::Literal::parse(&parse_context).map_err(|err| err.to_diagnostics(&parse_context))?,
        &lower::LowerContext::new(s),
    )
    .map_err(|err| RcMut::new(err.into()))
    .map(|lit| lit.value().clone())
}
