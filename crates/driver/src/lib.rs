// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad driver is a high-level API to be integrated in LSP, CLI or Viewer.

pub mod commands;
mod config;
pub mod document;
pub mod locate;
mod session;
mod watcher;

use microcad_lang::lower::Lower;
pub use microcad_lang::symbol::{Info, Symbol, SymbolInfo};
use microcad_lang::value::Value;

pub use microcad_lang_parse::ast;

pub mod base {
    pub use microcad_lang_base::{
        DiagRenderOptions, FormatTree, MICROCAD_EXTENSIONS, ResourceLocation, Source, Url,
    };
}

pub mod parse {
    pub use microcad_lang_parse::{Parse, ParseContext};
}

pub use microcad_lang::lower::ir;
pub mod lower {
    pub use microcad_lang::lower::LowerContext;
}

pub mod export {
    pub use microcad_export::*;
}

pub use microcad_lang::model::{Creator, Element, Model, OutputType};

pub use microcad_lang::render::{RenderCache, RenderContext, RenderResolution};
pub use microcad_lang_base::{
    ComputedHash, HashId, HashSet, Hashed, RcMut, Refer, SrcRef, SrcReferrer, Url,
};

pub use config::Config;
pub use document::Document;
pub use session::Session;
pub use watcher::Watcher;

/// We use [`miette::Result`] throught-out this crate.
pub type Result<T = ()> = miette::Result<T>;

/// Expose miette report.
pub use miette::Report;

/// Wrapper for miette macro
pub fn report(s: &str) -> Report {
    miette::miette!("{s}")
}

/// Parse a value from a string containing a literal.
pub fn value_from_str(s: &str) -> Result<Value> {
    let parse_context = parse::ParseContext::new(s);
    use parse::Parse;
    ir::Literal::lower(
        &ast::Literal::parse(&parse_context)?,
        &lower::LowerContext::new(s),
    )
    .map_err(|err| err.into())
    .map(|lit| lit.value().clone())
}

/// Install standard library (if it is not installed already).
pub fn install_std() -> Result {
    microcad_std::StdLib::new(microcad_std::StdLib::default_path())
        .map_err(|err| miette::miette!("Could not load standard library: {err}"))
        .map(|_| ())
}
