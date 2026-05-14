// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Cli, commands::RunCommand};

/// Check a µcad file for error but do not export anything.
#[derive(clap::Parser)]
pub struct Check {
    /// Input µcad file.
    input: String,
}

impl RunCommand<()> for Check {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::{Document, commands::Compile};
        let mut document = Document::open(&self.input)?;

        match document.compile(cli.compile_parameters("0.1mm")?) {
            Ok(_) => {
                eprintln!("✅ File is valid: {}", self.input);
            }
            Err(err) => {
                eprintln!("⚠️ File has issues:\n{err}");
                cli.print_diagnostics(&document);
            }
        }

        Ok(())
    }
}
