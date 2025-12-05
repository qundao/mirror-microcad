// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI completions command.

use std::io::stdout;

use clap::CommandFactory;
use clap_complete::{Shell, generate};

use crate::*;

#[derive(clap::Parser)]
pub struct Completions {
    /// Print completions for the given shell (instead of generating any icons).
    /// These can be loaded/stored permanently, but they can also be sourced directly, e.g.:
    ///
    ///  microcad completions fish | source # fish
    ///  source <(microcad completions zsh) # zsh
    #[clap(verbatim_doc_comment, id = "SHELL")]
    shell: Shell,
}

impl RunCommand for Completions {
    fn run(&self, _cli: &Cli) -> anyhow::Result<()> {
        generate(self.shell, &mut Cli::command(), "microcad", &mut stdout());
        Ok(())
    }
}
