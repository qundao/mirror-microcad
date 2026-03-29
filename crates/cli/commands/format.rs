// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::Read;

use miette::miette;

use crate::{Cli, commands::RunCommand};

/// Format a µcad file.
#[derive(clap::Parser)]

pub struct Format {
    /// Input µcad file.
    pub input: std::path::PathBuf,
}

impl RunCommand<()> for Format {
    fn run(&self, _cli: &Cli) -> miette::Result<()> {
        let source = std::fs::read_to_string(&self.input).map_err(|e| miette!("{e}"))?;

        let mut source = String::new();
        file.read_to_string(&mut source)
            .map_err(|e| miette!("{e}"))?;
        let tokens: Vec<_> = microcad_syntax::lex(&source).collect();
        let source_file = microcad_syntax::parse(&tokens).map_err(|errors| {
            miette!(
                "{errors}",
                errors = errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })?;

        println!("{}", microcad_lang_format::format(&source_file));

        Ok(())
    }
}
