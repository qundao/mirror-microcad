// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI watch command

use crate::*;

#[derive(clap::Parser)]
pub struct Watch {
    pub input: String,

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
        use microcad_driver::prelude as mu;
        use mu::traits::*;

        let mut watcher = mu::Watcher::new()?;
        let render_cache = mu::RcMut::new(mu::RenderCache::new());

        let compile_params = cli.compile_parameters();
        let render_params = cli
            .render_params(&self.resolution)?
            .with_cache(render_cache.clone());

        // Recompile whenever something relevant happens.
        loop {
            let export_params = mu::ExportParameters {
                input_path: std::path::PathBuf::from(&self.input),
                output_path: self.output.clone(),
                config: cli.config.export.clone(),
            };
            let mut document = mu::Document::open(&self.input)?;
            match document
                .compile(compile_params.clone())
                .and(document.render(render_params.clone()))
                .and(document.export(export_params))
            {
                Ok(exported_files) => {
                    eprint!("{exported_files}");
                }
                Err(err) => {
                    eprintln!("{err}");
                    cli.print_diagnostics(&document);
                }
            }

            // Watch all dependencies of the most recent compilation.
            watcher.update(vec![self.input.clone().into()])?;

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
