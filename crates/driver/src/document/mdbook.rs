// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, RcMut};
use microcad_lang_markdown::{MdBook, MdBookError};
use miette::Diagnostic;
use thiserror::Error;

use crate::{commands, document};

#[derive(Error, Debug, Diagnostic)]
pub enum MdBookUnitError {
    /// Mdbook is not loaded
    #[error("Mdbook is not loaded")]
    NotLoaded,

    /// Error when loading mdbook.
    #[error("Mdbook error: {0}")]
    MdBook(#[from] MdBookError),
}

/// State of a markdown book
#[derive(Default)]
pub struct State {
    mdbook: Option<MdBook>,
}

impl commands::LoadFromFile for document::MdBook {
    fn load_from_file(&self) -> document::Result {
        let state = &mut *self.state.borrow_mut();
        state.mdbook = Some(
            MdBook::new(self.try_file_path()?)
                .map_err(|err| RcMut::new(MdBookUnitError::MdBook(err).into()))?,
        );
        Ok(())
    }
}

impl commands::Format for document::MdBook {
    fn format(&self, params: &commands::FormatParameters) -> document::Result<bool> {
        commands::LoadFromFile::load_from_file(self)?;

        let state = &mut *self.state.borrow_mut();
        let mut formatted = false;
        let config = params;

        match &mut state.mdbook {
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
                    Err(RcMut::new(diags))
                } else {
                    Ok(formatted)
                }
            }
            None => Err(RcMut::new(MdBookUnitError::NotLoaded.into())),
        }
    }
}

impl commands::Sync for document::MdBook {
    fn sync(&self) -> document::Result {
        let state = &*self.state.borrow();
        match &state.mdbook {
            Some(mdbook) => {
                mdbook.save_all().expect("No error");
                Ok(())
            }
            None => Err(RcMut::new(MdBookUnitError::NotLoaded.into())),
        }
    }
}
