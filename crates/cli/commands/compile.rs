// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::ValueEnum;
use microcad_driver::prelude::base::{ArtifactKind, DiagRenderOptions};
use miette::IntoDiagnostic;

use crate::{Cli, commands::RunCommand};

/// Compile a µcad file and output artifacts for debugging
#[derive(clap::Parser)]
pub struct Compile {
    /// Input µcad file.
    input: String,
    /// Specify compiler artifacts to be emitted (e.g., --emit ast,ir)
    #[arg(short, long)]
    pub emit: Vec<ArtifactKind>,
}

impl RunCommand<()> for Compile {
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        use microcad_driver::prelude as mu;
        use mu::traits::*;

        let mut source_file = mu::SourceFile::load(&self.input)?;

        match source_file.compile(cli.compile_parameters()) {
            Ok(_) => {
                let path = source_file.source.path().expect("A path");

                for artifact_kind in &self.emit {
                    source_file.emit(&path, artifact_kind);
                }
            }
            Err(err) => {
                eprintln!("⚠️ File has issues:\n{err}");
                /*source_file.diagnostics().for_each(|diag| {
                    eprintln!(
                        "{}",
                        diag.render_to_string(&source_file.source, &DiagRenderOptions::default())
                    )
                });*/
            }
        }

        Ok(())
    }
}
