// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI doc command

use std::path::PathBuf;

use microcad_lang::diag::*;

use crate::{
    Cli,
    commands::{Resolve, RunCommand},
};

#[derive(clap::Parser)]
pub struct Doc {
    #[clap(flatten)]
    pub resolve: Resolve,

    /// Print model tree.
    #[clap(long)]
    pub output_path: Option<std::path::PathBuf>,
}

impl RunCommand<()> for Doc {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        // run prior parse step
        let context = self.resolve.run(cli)?;

        let output_path = self
            .output_path
            .clone()
            .unwrap_or(PathBuf::from("./doc").join(&self.resolve.parse.input_name()?.to_string()));

        match context.has_errors() {
            true => {
                eprintln!("Resolve failed:");
                eprintln!("{}", context.diagnosis());
                eprintln!("Documentation could not generated!");
                return Ok(());
            }
            false => log::info!("Successfully resolved!"),
        }

        let start = std::time::Instant::now();

        use microcad_markdown_support::book::BookWriter;
        let book_writer = BookWriter::new(output_path);
        book_writer
            .write(&context.root)
            .map_err(|err| miette::miette!("{err}"))?;

        if cli.time {
            eprintln!(
                "Doc generation Time: {}",
                Cli::time_to_string(&start.elapsed())
            );
        }

        Ok(())
    }
}
