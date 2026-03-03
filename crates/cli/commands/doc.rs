// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI doc command

use std::path::PathBuf;

use clap_complete::generator;
use microcad_builtin::Symbol;
use microcad_docgen::*;
use microcad_lang::diag::*;

use crate::{
    Cli,
    commands::{Parse, Resolve, RunCommand},
};

#[derive(clap::Parser)]
pub struct Doc {
    /// Input µcad file.
    input: Option<std::path::PathBuf>,

    /// Build documentation for an external library.
    lib: Option<String>,

    /// Generator (md (default), mdbook).
    generator: Option<String>,

    /// Output path for markdown book
    #[clap(long)]
    pub output_path: Option<std::path::PathBuf>,
}

impl Doc {
    /// Generator from arguments
    fn generator(&self) -> Box<dyn DocGen> {
        let name = self.generator.clone().unwrap_or("md".to_string());

        match name.as_str() {
            "md" => Box::new(Md {
                _output_file: self.output_path.clone(),
            }),
            "mdbook" => Box::new(MdBook {
                path: self.output_path.clone().unwrap_or_default(),
            }),
            _ => {
                panic!("No generator with name `{name}`");
            }
        }
    }

    /// Resolve symbol from arguments
    fn symbol(&self) -> Symbol {
        todo!()
    }
}

impl RunCommand<()> for Doc {
    fn run(&self, _cli: &Cli) -> miette::Result<()> {
        let generator = self.generator();
        let symbol = self.symbol();

        Ok(generator
            .doc_gen(&symbol)
            .map_err(|err| miette::miette!("{err}"))?)
    }
}
