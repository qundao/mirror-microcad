// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad driver is a high-level API to be integrated in LSP, CLI or Viewer.

pub mod commands;
mod config;
mod document;
mod session;
mod watcher;

use microcad_lang::lower::Lower;
use microcad_lang::value::Value;

pub use microcad_lang::lower::ir::Source;
pub use microcad_lang::model::Model;
pub use microcad_lang::render::{RenderCache, RenderContext};
pub use microcad_lang_base::{RcMut, Url};

pub use config::Config;
pub use document::Document;
use microcad_lang_parse::{Parse, ParseContext};
pub use session::Session;
pub use watcher::Watcher;

/// Parse a value from a string containing a literal.
pub fn value_from_str(s: &str) -> document::Result<Value> {
    let parse_context = ParseContext::new(s);
    microcad_lang::lower::ir::Literal::lower(
        &microcad_lang_parse::ast::Literal::parse(&parse_context)
            .map_err(|err| err.to_diagnostics(&parse_context))?,
        &microcad_lang::lower::LowerContext::new(s),
    )
    .map_err(|err| RcMut::new(err.into()))
    .map(|lit| lit.value().clone())
}
