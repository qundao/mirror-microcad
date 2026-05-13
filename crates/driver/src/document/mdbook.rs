// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, RcMut, ResourceLocation, Url};
use microcad_lang_markdown::{MdBook, MdBookError};
use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

use crate::{
    Result, commands,
    document::{self, CaptureDiags, TryFilePath},
};

#[derive(Error, Debug, Diagnostic)]
pub enum MdBookUnitError {
    /// Mdbook is not loaded
    #[error("Mdbook is not loaded")]
    NotLoaded,

    /// Error when loading mdbook.
    #[error("Mdbook error: {0}")]
    MdBook(#[from] MdBookError),
}

/// A markdown book document
pub struct MdBookDocument {
    url: Url,
    mdbook: Option<MdBook>,
    diags: RcMut<Diagnostics>,
}

impl MdBookDocument {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            mdbook: None,
            diags: RcMut::new(Default::default()),
        }
    }
}

impl ResourceLocation for MdBookDocument {
    fn url(&self) -> &Url {
        &self.url
    }
}

impl TryFilePath for MdBookDocument {}

impl CaptureDiags for MdBookDocument {
    fn diags(&self) -> RcMut<Diagnostics> {
        self.diags.clone()
    }
}

impl commands::LoadFromFile for document::MdBook {
    fn load_from_file(&mut self) -> Result {
        self.mdbook = Some(MdBook::new(self.try_file_path()?).into_diagnostic()?);
        Ok(())
    }
}

impl commands::Format for document::MdBook {
    fn format(&mut self, params: &commands::FormatParameters) -> Result<bool> {
        let mut formatted = false;
        let config = params;

        match &mut self.mdbook {
            Some(mdbook) => {
                let mut diags = Diagnostics::default();

                mdbook
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

                if diags.has_errors() {
                    self.diags().replace(diags);
                    Err(miette::miette!("Error formatting mdbook"))
                } else {
                    Ok(formatted)
                }
            }
            None => Err(MdBookUnitError::NotLoaded.into()),
        }
    }
}

impl commands::Sync for document::MdBook {
    fn sync(&self) -> Result {
        match &self.mdbook {
            Some(mdbook) => mdbook.save_all().into_diagnostic(),
            None => Err(MdBookUnitError::NotLoaded.into()),
        }
    }
}
