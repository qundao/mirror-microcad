// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI doc command

use microcad_builtin::Symbol;
use microcad_docgen::*;
use microcad_lang::syntax::Identifier;

use crate::{
    Cli,
    commands::{Resolve, RunCommand},
};

/// Generate documentation from code.
#[derive(clap::Parser)]
pub struct Doc {
    /// Input file or library name.
    ///
    /// Build documentation for an external library (only `__builtin` and `std` are possible).
    #[clap(flatten)]
    resolve: Resolve,

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
                _output_file: self.output.clone(),
            })),
            "mdbook" => Ok(Box::new(MdBook {
                path: self.output.clone().unwrap_or_default(),
            })),
            _ => Err(miette::miette!("No generator with name `{name}`")),
        }
    }

    /// Resolve symbol from arguments
    fn symbol(&self, cli: &Cli) -> miette::Result<Symbol> {
        let input = &self.resolve.parse.input;
        // Handle special case for builtin symbol.
        if let Some(s) = input.to_str()
            && s == "__builtin"
        {
            return Ok(microcad_builtin::builtin_module());
        }

        let context = self.resolve.run(cli)?;
        let symbol = context
            .root
            .get_child(&Identifier::no_ref("mod")) // FIXME. This symbol should have same name as its parent directory (e.g. `std`)
            .expect("Symbol");

        Ok(symbol)
    }
}

impl RunCommand<()> for Doc {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        let generator = self.generator()?;
        let symbol = self.symbol(cli)?;
        Ok(generator
            .doc_gen(&symbol)
            .map_err(|err| miette::miette!("{err}"))?)
    }
}
