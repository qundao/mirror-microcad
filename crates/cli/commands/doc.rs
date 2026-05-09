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
        let document = Document::from_file_path(&self.input)?;
        use microcad_driver::commands::{DocGen, DocGenParameters};
        let params = DocGenParameters {
            generator_id: self.generator.clone(),
            output_path: self.output.clone(),
        };

        match document.doc_gen(&params) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error generating documentation:");
                cli.print_diagnostics(&document);
            }
        }

        Ok(())
    }
}
