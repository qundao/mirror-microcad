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

    /// Omit export.
    #[arg(short, long)]
    pub dry_run: bool,
}

impl RunCommand for Export {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        let document = Document::from_file_path(&self.input, cli.config.clone())?;
        todo!();
        /*

        match document {
            Document::Source(item) => {
                item.render(
                    RenderResolution { linear: 0.1 }, /*self.resolution */
                    None,
                );
                item.export(self.output.clone()).unwrap().export();
                Ok(())
            }
            Document::Markdown(_) => miette::bail!("Export for markdown is not implemented"),
            Document::MdBook(_) => miette::bail!("Export for mdbook is not implemented"),
            Document::Builtin(_) => miette::bail!("Export for builtin is not implemented"),
        }*/
    }
}

impl Export {}
