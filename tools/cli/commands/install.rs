// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI install command.

use crate::*;

#[derive(clap::Parser)]
pub struct Install {
    /// Name of µcad library to install (currently only `std` is supported).
    pub library: String,

    /// Directory to install libraries into.
    /// If this command line option is not set, the library will be installed in ~/.microcad/lib/$LIBNAME
    pub root: Option<std::path::PathBuf>,

    /// Force overwrite.
    #[arg(short = 'f', long, default_value = "false", action = clap::ArgAction::SetTrue)]
    force: bool,
}

impl RunCommand for Install {
    fn run(&self, _cli: &Cli) -> anyhow::Result<()> {
        if self.library == "std" {
            Ok(microcad_std::extract(self.force)?)
        } else {
            Err(anyhow::anyhow!(
                "Only `std` is supported as installable library at the moment.",
            ))
        }
    }
}
