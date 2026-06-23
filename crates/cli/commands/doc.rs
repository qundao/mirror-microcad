// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI doc command

use crate::{Cli, commands::RunCommand};

/// Generate documentation from code.
#[derive(clap::Parser)]
pub struct Doc {
    /// Input file or library name.
    ///
    /// Build documentation for an external library (only `__builtin` and `std` are possible).
    input: String,

    /// Generator (md (default), mdbook).
    #[arg(short = 'g', long = "generator")]
    generator: Option<String>,

    /// Output path for markdown book
    #[arg(short = 'o', long)]
    output: Option<std::path::PathBuf>,

    /// Do not add any external search paths.
    /// This only used for building docs for standard library.
    /// *NOTE: This CLI argument is supposed to be removed*
    #[arg(long)]
    no_std: bool,
}

impl RunCommand<()> for Doc {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::prelude as mu;
        use mu::traits::*;

        let mut document = mu::Document::open(&self.input)?;
        let params = mu::DocGenParameters {
            generator_id: self.generator.clone(),
            output_path: self.output.clone(),
            resolve_parameters: match self.no_std {
                true => mu::ResolveParameters {
                    search_paths: vec![],
                    no_builtin: true,
                },
                false => mu::ResolveParameters::default(),
            },
        };

        match document.doc_gen(params) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error generating documentation:\n{err}");
                cli.print_diagnostics(&document);
            }
        }

        Ok(())
    }
}
