// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad driver is a high-level API to be integrated in LSP, CLI or Viewer.

pub mod commands;
mod config;
pub mod document;
pub mod locate;
pub mod prelude;
mod session;
mod watcher;

use microcad_lang::value::Value;

/// We use [`miette::Result`] throught-out this crate.
pub type Result<T = ()> = miette::Result<T>;

/// Expose miette report.
pub use miette::Report;

/// Wrapper for miette macro
pub fn report(s: &str) -> Report {
    miette::miette!("{s}")
}

pub use config::Config;

/// Parse a value from a string containing a literal.
pub fn value_from_str(s: &str) -> Result<Value> {
    use mu::traits::*;
    use prelude as mu;

    let parse_context = prelude::parse::ParseContext::new(s);
    mu::ir::Literal::lower(
        &mu::ast::Literal::parse(&parse_context)?,
        &mu::lower::LowerContext::new(s),
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
