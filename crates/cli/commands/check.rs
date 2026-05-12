// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_driver::Document;

use crate::{Cli, commands::RunCommand};

#[derive(clap::Parser)]
pub struct Check {
    input: std::path::PathBuf,
}

impl RunCommand<()> for Check {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::commands::Check;
        let mut document = Document::from_file_path(&self.input)?;

        match document.check(cli.config.as_ref()) {
            Ok(true) => {
                eprintln!("File is ok.")
            }
            Ok(false) | Err(_) => {
                eprintln!("File has issues:");
                cli.print_diagnostics(&document);
            }
        }

        Ok(())
    }
}
