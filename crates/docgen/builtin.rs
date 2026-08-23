// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Documentation generator for µcad built-ins.

use clap::Parser;
use microcad_docgen as docgen;
use miette::IntoDiagnostic;

/// µcad built-in doc generater CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct BuiltinCli {
    /// Output path (must be a directory).
    output: std::path::PathBuf,
}

fn main() -> miette::Result<()> {
    let cli = BuiltinCli::parse();
    docgen::BuiltinMdbook::new()
        .write(cli.output)
        .into_diagnostic()
}
