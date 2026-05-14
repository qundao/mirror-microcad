// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use crate::{Cli, commands::RunCommand};

/// Parse and evaluate and export a µcad file.
#[derive(clap::Parser)]
pub struct Export {
    pub input: String,

    /// Output file (e.g. an SVG or STL).
    pub output: Option<std::path::PathBuf>,

    /// The resolution of this export.
    ///
    /// The resolution can changed relatively `200%` or to an absolute value `0.05mm`.
    #[arg(short, long, default_value = "0.1mm")]
    pub resolution: String,

    /// List all export target files.
    #[arg(short, long)]
    pub dry_run: bool,
}

impl RunCommand for Export {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::Document;
        use microcad_driver::commands::{Compile, Export as _, ExportParameters};

        let mut document = Document::open(&self.input)?;

        let params = ExportParameters {
            input_path: std::path::PathBuf::from(&self.input),
            output_path: self.output.clone(),
            config: cli.config.export.clone(),
        };

        match document
            .compile(cli.compile_parameters(&self.resolution)?)
            .and(document.get_export_targets(&params))
        {
            Ok(targets) => {
                if self.dry_run {
                    eprintln!("{targets}");
                } else {
                    match targets.export() {
                        Ok(exported_files) => {
                            eprint!("{exported_files}");
                        }
                        Err(err) => {
                            eprintln!("{err}");
                            cli.print_diagnostics(&document);
                        }
                    }
                }
            }
            Err(_) => {
                cli.print_diagnostics(&document);
            }
        }
        Ok(())
    }
}
