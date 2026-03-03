// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI doc command

use microcad_builtin::Symbol;
use microcad_docgen::*;
use microcad_lang::{diag::*, resolve::ResolveContext};

use crate::{Cli, commands::RunCommand};

/// Generate documentation from code.
#[derive(clap::Parser)]
pub struct Doc {
    /// Input file or library name.
    ///
    /// Build documentation for an external library (only `__builtin` and `std` are possible).
    input: Option<std::path::PathBuf>,

    /// Generator (md (default), mdbook).
    #[arg(short = 'g', long = "generator")]
    generator: Option<String>,

    /// Output path for markdown book
    #[clap(long)]
    pub output_path: Option<std::path::PathBuf>,
}

impl Doc {
    /// Generator from arguments
    fn generator(&self) -> miette::Result<Box<dyn DocGen>> {
        let name = self.generator.clone().unwrap_or("md".to_string());

        match name.as_str() {
            "md" => Ok(Box::new(Md {
                _output_file: self.output_path.clone(),
            })),
            "mdbook" => Ok(Box::new(MdBook {
                path: self.output_path.clone().unwrap_or_default(),
            })),
            _ => Err(miette::miette!("No generator with name `{name}`")),
        }
    }

    fn resolve(&self, path: impl AsRef<std::path::Path>, no_std: bool) -> miette::Result<Symbol> {
        let path = path.as_ref();
        let root = microcad_lang::syntax::SourceFile::load(path)?;
        let search_path = if no_std {
            vec![]
        } else {
            vec![microcad_std::global_library_search_path()]
        };
        let context = ResolveContext::create(
            root,
            &search_path,
            Some(microcad_builtin::builtin_module()),
            DiagHandler::default(),
        )?;

        Ok(context.root)
    }

    /// Resolve symbol from arguments
    fn symbol(&self) -> miette::Result<Symbol> {
        let input = self
            .input
            .clone()
            .unwrap_or(std::env::current_dir().map_err(|err| miette::miette!("{err}"))?);

        match input.to_str() {
            Some(str) => match str {
                "__builtin" => Ok(microcad_builtin::builtin_module()),
                "std" => self.resolve(microcad_std::StdLib::default_path(), true),
                _ => self.resolve(input, false),
            },
            None => self.resolve(input, false),
        }
    }
}

impl RunCommand<()> for Doc {
    fn run(&self, _cli: &Cli) -> miette::Result<()> {
        let generator = self.generator()?;
        let symbol = self.symbol()?;
        Ok(generator
            .doc_gen(&symbol)
            .map_err(|err| miette::miette!("{err}"))?)
    }
}
