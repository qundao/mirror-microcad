// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::Diagnostics;
use microcad_lang_format::FormatConfig;

use crate::{Cli, commands::RunCommand};

/// Format a µcad file.
#[derive(clap::Parser)]

pub struct Format {
    /// Input µcad file.
    pub input: std::path::PathBuf,
}

/// High-level API to format an entire mdbook.
pub fn format_mdbook(
    mdbook: &mut microcad_lang_markdown::MdBook,
    config: &FormatConfig,
) -> Result<(), Diagnostics> {
    let mut diagnostics = Diagnostics::default();

    // 1. Iterate over code blocks. 'path' is the PathBuf of the .md file.
    mdbook
        .code_blocks_mut()
        .filter(|(_, code_block)| code_block.can_format())
        .for_each(|(path, code_block)| {
            if let Err(err) = microcad_lang_format::format_str(&code_block.code, config) {
                diagnostics.append(err);
            } else if let Ok(formatted) = microcad_lang_format::format_str(&code_block.code, config)
            {
                // Only update the code if formatting succeeded
                code_block.code = formatted;
            }
        });

    // 3. If we hit issues, return the map in the specific variant
    if diagnostics.has_errors() {
        return Err(diagnostics);
    }

    Ok(())
}

impl RunCommand<()> for Format {
    fn run(&self, _cli: &Cli) -> miette::Result<()> {
        let config = FormatConfig::default();
        use miette::miette;

        // Check if the input is a mdbook configuration
        if self.input.ends_with("book.toml") {
            let mut mdbook =
                microcad_lang_markdown::MdBook::new(&self.input).map_err(|err| miette!("{err}"))?;
            format_mdbook(&mut mdbook, &config).expect("Error handling");
            // 2. Persist the successfully formatted parts to disk
            mdbook.save_all().map_err(|err| miette!("{err}"))?;

            eprintln!("Formatted mdbook in {:?}", mdbook.src_path);
            Ok(())
        } else {
            // Standard single-file formatting logic
            let source = std::fs::read_to_string(&self.input)
                .map_err(|e| miette!("Failed to read {}: {}", self.input.display(), e))?;

            let formatted =
                microcad_lang_format::format_str(&source, &config).expect("Error handling");

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
