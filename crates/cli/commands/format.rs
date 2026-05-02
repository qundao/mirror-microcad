// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    fn run(&self, cli: &Cli) -> miette::Result<()> {
        let document = microcad_driver::Document::from_file_path(&self.input, cli.config.clone())?;

        if let Err(_) = document.load_from_file().and_then(|_| {
            if document.format()? {
                document.sync()
            } else {
                Ok(())
            }
        }) {
            eprintln!("{}", document.diagnostics_string());
        }

        Ok(())
    }
}
