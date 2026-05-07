// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI parse command.

use crate::*;

use microcad_lang::lower::ir;
use std::rc::Rc;

#[derive(clap::Parser)]
pub struct Parse {
    /// Input µcad file.
    pub input: std::path::PathBuf,

    /// Print syntax tree.
    #[clap(long)]
    pub syntax: bool,
}

impl Parse {
    pub fn input_with_ext(&self, cli: &Cli) -> std::path::PathBuf {
        cli.path_with_default_ext(&self.input)
    }
}

impl RunCommand<Rc<ir::SourceFile>> for Parse {
    fn run(&self, cli: &Cli) -> miette::Result<Rc<ir::SourceFile>> {
        let start = std::time::Instant::now();
        let source_file = ir::SourceFile::load(self.input_with_ext(cli))?;

        if cli.time {
            eprintln!("Parsing Time   : {}", Cli::time_to_string(&start.elapsed()));
        }

        if cli.is_parse() {
            eprintln!("Parsed successfully!");
        }

        if self.syntax {
            println!("{source_file:?}");
        }
        Ok(source_file)
    }
}
