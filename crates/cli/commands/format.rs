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
        let document = Document::from_file_path(&self.input)?;
        use microcad_driver::commands::{Format, FormatParameters, LoadFromFile, Sync};

        if document
            .load_from_file()
            .and_then(|_| {
                let params = FormatParameters::default();
                if document.format(&params)? {
                    document.sync()
                } else {
                    Ok(())
                }
            })
            .is_err()
        {
            cli.print_diagnostics(&document);
        }

        Ok(())
    }
}
