// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use std::str::FromStr;

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

    /// Omit export.
    #[arg(short, long)]
    pub dry_run: bool,
}

impl RunCommand for Export {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::commands::{
            Export as _, GetExportTargetParameters, Pipeline, Render, RenderParameters,
        };

        let document = Document::from_file(&self.input)?;

        match document {
            Document::Source(mut source) => {
                let params = GetExportTargetParameters {
                    input_path: self.input.clone(),
                    output_path: self.output.clone(),
                    config: cli.config.export.clone(),
                };

                match source
                    .run_pipeline(&cli.config)
                    .and(source.render(&RenderParameters::from_str(&self.resolution)?))
                    .and(source.get_export_targets(&params))
                {
                    Ok(targets) => {
                        targets.export()?;
                    }
                    Err(_) => {
                        eprintln!("Error export documentation:");
                        cli.print_diagnostics(&source);
                    }
                }
                Ok(())
            }
            Document::Markdown(_) => miette::bail!("Export for markdown is not implemented"),
            Document::MdBook(_) => miette::bail!("Export for mdbook is not implemented"),
            Document::Builtin(_) => miette::bail!("Export for builtin is not implemented"),
        }
    }
}
