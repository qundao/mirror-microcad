// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, SourceLocation, Url};
use microcad_lang_markdown::MdBook;
use miette::IntoDiagnostic;

use crate::{
    Result, commands,
    document::{self, CaptureDiags},
};

/// A markdown book document
pub struct MdBookDocument {
    /// Location of the `book.toml`
    pub location: SourceLocation,
    /// The loaded MdBook
    mdbook: MdBook,
    /// Diagnostics
    diags: Diagnostics,
}

impl MdBookDocument {
    pub fn new(url: Url) -> Result<Self> {
        let location = SourceLocation::new(url);
        Ok(Self {
            mdbook: MdBook::new(location.path().ok_or(miette::miette!("Not a file path"))?)
                .into_diagnostic()?,
            location,
            diags: Default::default(),
        })
    }
}

impl CaptureDiags for MdBookDocument {
    fn diags(&self) -> &Diagnostics {
        &self.diags
    }

    fn diags_mut(&mut self) -> &mut Diagnostics {
        &mut self.diags
    }
}

impl commands::Format for document::MdBook {
    fn format(&mut self, params: &commands::FormatParameters) -> Result<bool> {
        let mut formatted = false;
        let config = params;
        self.diags.clear();
        let mut diags = Diagnostics::default();

        self.mdbook
            .code_blocks_mut()
            .filter(|(_, code_block)| code_block.can_format())
            .for_each(|(_, code_block)| {
                match microcad_lang_format::format_str(&code_block.code, config) {
                    Ok(code) => {
                        formatted |= code_block.code != code;
                        code_block.code = code;
                    }
                    Err(diag) => {
                        diags.append(diag);
                    }
                }
            });

        self.diags.append(diags);

        if self.diags().has_errors() {
            Err(miette::miette!("Error formatting mdbook"))
        } else {
            Ok(formatted)
        }
    }
}

impl commands::Sync for document::MdBook {
    fn sync(&self) -> Result {
        Ok(self.mdbook.save_all().into_diagnostic()?)
    }
}
