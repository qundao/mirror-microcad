// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use microcad_driver::Document;

use crate::{Cli, commands::RunCommand};

/// Parse and evaluate and export a µcad file.
#[derive(clap::Parser)]
pub struct Export {
    pub input: std::path::PathBuf,

    /// Output file (e.g. an SVG or STL).
    pub output: Option<std::path::PathBuf>,

    /// The resolution of this export.
    ///
    /// The resolution can changed relatively `200%` or to an absolute value `0.05mm`.
    #[arg(short, long, default_value = "0.1mm")]
    pub resolution: String,

    /// List all export target files.
    #[arg(short, long)]
    pub targets: bool,
}

impl RunCommand for Export {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::commands::{Compile, Export as _, ExportParameters};

        let mut document = Document::from_file(&self.input)?;

        let params = ExportParameters {
            input_path: self.input.clone(),
            output_path: self.output.clone(),
            config: cli.config.export.clone(),
        };

        match document
            .compile(cli.compile_parameters(&self.resolution)?)
            .and(document.get_export_targets(&params))
        {
            Ok(targets) => {
                targets.export()?;
            }
            Err(_) => {
                eprintln!("Error export documentation:");
                cli.print_diagnostics(&document);
            }
        }
        Ok(())
    }
}
