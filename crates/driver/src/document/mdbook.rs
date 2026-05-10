// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, ResourceLocation};
use microcad_lang_markdown::{MdBook, MdBookError};
use miette::Diagnostic;
use thiserror::Error;
use url::Url;

use crate::{commands, document};

#[derive(Error, Debug, Diagnostic)]
pub enum MdBookUnitError {
    /// Error loading mdbook toml from file.
    #[error("Mdbook is not local: {0}")]
    NoLocalMdBook(Url),

    /// Mdbook is not loaded
    #[error("Mdbook is not loaded")]
    NotLoaded,

    /// Error when loading mdbook.
    #[error("Mdbook error: {0}")]
    MdBook(#[from] MdBookError),
}

/// State of a markdown book
#[derive(Default)]
pub enum State {
    #[default]
    Raw,
    Loaded {
        mdbook: MdBook,
    },
}

impl commands::LoadFromFile for document::MdBookAsset {
    fn load_from_file(&self) -> document::Result {
        self.transition(|_| match self.to_file_path() {
            Some(path) => {
                let mdbook = MdBook::new(path).map_err(MdBookUnitError::MdBook)?;
                Ok(State::Loaded { mdbook })
            }
            None => Err(MdBookUnitError::NoLocalMdBook(self.url.clone()).into()),
        })
    }
}

impl commands::Format for document::MdBookAsset {
    fn format(&self, params: &commands::FormatParameters) -> document::Result<bool> {
        commands::LoadFromFile::load_from_file(self)?;
        let mut formatted = false;
        let config = params;

        self.transition(|state| {
            let mut diags = Diagnostics::default();

            if let State::Loaded { mut mdbook } = state {
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
                    Err(diags)
                } else {
                    Ok(State::Loaded { mdbook })
                }
            } else {
                Err(MdBookUnitError::NotLoaded.into())
            }
        })?;

        Ok(formatted)
    }
}

impl commands::Sync for document::MdBookAsset {
    fn sync(&self) -> document::Result {
        match &*self.state.borrow() {
            State::Raw => (),
            State::Loaded { mdbook } => mdbook.save_all().expect("No error"),
        }
        Ok(())
    }
}
