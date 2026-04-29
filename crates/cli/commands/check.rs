// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Cli, commands::RunCommand};

#[derive(clap::Parser)]
pub struct Check {
    input: std::path::PathBuf,
}

impl RunCommand<()> for Check {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        todo!()
    }
}
