// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI watch command

use microcad_driver::{Document, RcMut, RenderCache};

use crate::*;

#[derive(clap::Parser)]
pub struct Watch {
    pub input: std::path::PathBuf,

    /// Output file (e.g. an SVG or STL).
    pub output: Option<std::path::PathBuf>,

    /// The resolution of this export.
    ///
    /// The resolution can changed relatively `200%` or to an absolute value `0.05mm`.
    #[arg(short, long, default_value = "0.1mm")]
    pub resolution: String,
}

/// Run this command for a CLI.
impl RunCommand for Watch {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::commands::{
            Export, GetExportTargetParameters, Pipeline, Render, RenderParameters,
        };
        use std::str::FromStr;

        let mut watcher = microcad_driver::Watcher::new()?;
        let input = cli.config.path_with_default_ext(&self.input);
        let render_cache = RcMut::new(RenderCache::new());

        let render_params =
            RenderParameters::from_str(&self.resolution)?.with_cache(render_cache.clone());
        let export_params = GetExportTargetParameters {
            input_path: self.input.clone(),
            output_path: self.output.clone(),
            config: cli.config.export.clone(),
        };

        // Recompile whenever something relevant happens.
        loop {
            let mut document = Document::from_file(&input)?;
            match document
                .run_pipeline(&cli.config)
                .and(document.render(&render_params))
                .and(document.export(&export_params))
            {
                Ok(exported_files) => {
                    eprintln!("{exported_files}");
                }
                Err(err) => {
                    eprintln!("{err}");
                    cli.print_diagnostics(&document);
                }
            }

            // Watch all dependencies of the most recent compilation.
            watcher.update(vec![input.clone()])?;

            // Remove unused cache items.
            {
                let mut cache = render_cache.borrow_mut();
                cache.garbage_collection();
            }

            // Wait until anything relevant happens.
            watcher.wait()?;
        }
    }
}
