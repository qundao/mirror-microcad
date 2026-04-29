// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Cli, commands::RunCommand};

/// Format a µcad file.
#[derive(clap::Parser)]

pub struct Format {
    /// Input µcad file.
    pub input: std::path::PathBuf,
}

impl RunCommand<()> for Format {
    fn run(&self, _cli: &Cli) -> miette::Result<()> {
        use microcad_lang_format::FormatConfig;

        let config = FormatConfig::default();
        use miette::miette;

        // Check if the input is a mdbook configuration
        if self.input.ends_with("book.toml") {
            let mut mdbook = microcad_lang_markdown::MdBookDirectory::new(&self.input)
                .map_err(|err| miette!("{err}"))?;
            microcad_lang_format::format_mdbook(&mut mdbook, &config)
                .map_err(|err| miette!("{err}"))?;

            eprintln!("Formatted mdbook in {:?}", mdbook.src_path);
            Ok(())
        } else {
            // Standard single-file formatting logic
            let source = std::fs::read_to_string(&self.input)
                .map_err(|e| miette!("Failed to read {}: {}", self.input.display(), e))?;

            let formatted = microcad_lang_format::format_str(&source, &config)
                .map_err(|err| miette!("{err}"))?;

            if source == formatted {
                eprintln!(
                    "File `{}` is already formatted. No changes have been made.",
                    self.input.display()
                );
            } else {
                std::fs::write(&self.input, formatted).map_err(|err| miette!("{err}"))?;
                eprintln!("Successfully formatted file `{}`", self.input.display());
            }

            Ok(())
        }
    }
}
