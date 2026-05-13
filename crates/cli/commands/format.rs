// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_driver::Document;

use crate::{Cli, commands::RunCommand};

/// Format a µcad file.
#[derive(clap::Parser)]

pub struct Format {
    /// Input µcad file.
    pub input: std::path::PathBuf,
}

impl RunCommand<()> for Format {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::commands::{Format, FormatParameters, Pipeline, Sync};
        let mut document = Document::from_file(&self.input)?;
        let params = FormatParameters::default();

        match document.parse().and(document.format(&params)) {
            Ok(true) => {
                document.sync()?;
                eprintln!("Formatted document.");
            }
            Ok(false) => {
                eprintln!("Document has been already formatted.");
            }
            Err(_) => {
                cli.print_diagnostics(&document);
            }
        }

        Ok(())
    }
}
