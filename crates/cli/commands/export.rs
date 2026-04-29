// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use microcad_driver::{Document, ExportCommand, Model};

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

impl RunCommand<Vec<(Model, ExportCommand)>> for Export {
    fn run(&self, cli: &Cli) -> miette::Result<Vec<(Model, ExportCommand)>> {
        let export = Document::new(self.input.clone())
            .load()?
            .export(cli.session.config.export.clone(), self.output.clone())?;

        if self.targets {
            export.list_targets(&export.target_models()?)?;
        }

        export.export()
    }
}

impl Export {}
