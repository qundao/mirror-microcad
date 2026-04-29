// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI doc command

use crate::{Cli, commands::RunCommand};

use microcad_docgen::*;
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

impl Doc {
    /// Generator from arguments
    fn generator(&self) -> miette::Result<Box<dyn DocGen>> {
        let name = self.generator.clone().unwrap_or("md".to_string());

        match name.as_str() {
            "md" => Ok(Box::new(Md {
                output_path: self.output.clone(),
            })),
            "mdbook" => Ok(Box::new(MdBook {
                path: self.output.clone().unwrap_or_default(),
            })),
            _ => Err(miette::miette!("No generator with name `{name}`")),
        }
    }
}

impl RunCommand<()> for Doc {
    fn run(&self, _cli: &Cli) -> miette::Result<()> {
        let generator = self.generator()?;
        let symbol = Document::new(self.input.clone()).load()?.symbol()?;
        generator
            .doc_gen(&symbol)
            .map_err(|err| miette::miette!("{err}"))
    }
}
