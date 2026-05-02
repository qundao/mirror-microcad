// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI doc command

use crate::{Cli, commands::RunCommand};

use microcad_driver::Document;

/// Generate documentation from code.
#[derive(clap::Parser)]
pub struct Doc {
    /// Input file or library name.
    ///
    /// Build documentation for an external library (only `__builtin` and `std` are possible).
    input: std::path::PathBuf,

    /// Generator (md (default), mdbook).
    #[arg(short = 'g', long = "generator")]
    generator: Option<String>,

    /// Output path for markdown book
    #[arg(short = 'o', long)]
    output: Option<std::path::PathBuf>,
}

impl RunCommand<()> for Doc {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        let document = Document::from_file_path(&self.input, cli.config.clone())?;
        match document {
            Document::Source(item) => item.doc_gen(self.generator.clone(), self.output.clone()),
            Document::Builtin(item) => item.doc_gen(self.output.as_ref().unwrap().clone()),
            Document::Markdown(_) => miette::bail!("Cannot generate docs for markdown"),
            Document::MdBook(_) => miette::bail!("Cannot generate docs for mdbook"),
        }
    }
}
