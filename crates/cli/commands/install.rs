// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI install command.

use crate::*;
use miette::IntoDiagnostic;

#[derive(clap::Parser)]
pub struct Install {
    /// Name of µcad library to install (currently only `std` is supported).
    pub library: String,

    /// Force overwrite.
    #[arg(short = 'f', long, default_value = "false", action = clap::ArgAction::SetTrue)]
    force: bool,
}

impl RunCommand for Install {
    fn run(&self, _cli: &Cli) -> miette::Result<()> {
        if self.library == "std" {
            Ok(
                microcad_std::install(microcad_std::global_std_path(), self.force)
                    .into_diagnostic()?,
            )
        } else {
            miette::bail!("Only `std` is supported as installable library at the moment.")
        }
    }
}
