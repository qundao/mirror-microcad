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
        let mut watcher = microcad_driver::Watcher::new()?;
        let render_cache = RcMut::new(RenderCache::default());
        let input = cli.config.path_with_default_ext(&self.input);
        let document = Document::from_file_path(input, cli.config.clone())?;

        let document = match document {
            Document::Source(item) => item,
            Document::Markdown(item) => todo!(),
            Document::MdBook(item) => todo!(),
            Document::Builtin(item) => todo!(),
        };

        // Recompile whenever something relevant happens.
        loop {
            /*
            document.load_from_file();
            document.eval();
            document.render(RenderResolution { linear: 0.1 }, Some(cache));
            document.export();
            */
            todo!();
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
