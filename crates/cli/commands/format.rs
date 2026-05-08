// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Cli, commands::RunCommand};

/// Format a µcad file.
#[derive(clap::Parser)]

pub struct Format {
    /// Input µcad file.
    pub input: std::path::PathBuf,
}

impl RunCommand<()> for Format {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        let document = microcad_driver::Document::from_file_path(&self.input, cli.config.clone())?;

        if let Err(_) = document.load_from_file().and_then(|_| {
            if document.format()? {
                document.sync()
            } else {
                Ok(())
            }
        }) {
            eprintln!("{}", document.diagnostics_string());
        }

        Ok(())
    }
}
