// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, RcMut};
use miette::Diagnostic;
use thiserror::Error;

use crate::{commands, document, document::TryFilePath};
use microcad_lang_markdown::{Markdown, MarkdownError};

#[derive(Error, Debug, Diagnostic)]
pub enum MarkdownItemError {
    /// Mdbook is not loaded
    #[error("Mdbook is not loaded")]
    NotLoaded,

    /// Error when loading mdbook.
    #[error("Markdown error: {0}")]
    MarkdownError(#[from] MarkdownError),
}

#[derive(Default)]
pub struct State {
    markdown: Option<Markdown>,
}

impl commands::LoadFromFile for document::Markdown {
    fn load_from_file(&mut self) -> document::Result {
        let state = &mut *self.state.borrow_mut();
        state.markdown = Some(
            Markdown::load(self.try_file_path()?)
                .map_err(|err| RcMut::new(MarkdownItemError::MarkdownError(err).into()))?,
        );
        Ok(())
    }
}

impl commands::Format for document::Markdown {
    fn format(&mut self, params: &commands::FormatParameters) -> document::Result<bool> {
        let mut formatted = false;
        let config = params;
        let state = &mut *self.state.borrow_mut();
        let mut diags = Diagnostics::default();

        match &mut state.markdown {
            Some(markdown) => {
                markdown
                    .code_blocks_mut()
                    .filter(|code_block| code_block.can_format())
                    .for_each(|code_block| {
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
            None => Err(RcMut::new(MarkdownItemError::NotLoaded.into())),
        }
    }
}

impl commands::Sync for document::Markdown {
    fn sync(&self) -> document::Result {
        let state = &*self.state.borrow();
        match &state.markdown {
            Some(markdown) => {
                markdown
                    .save(self.try_file_path()?)
                    .expect("Error handling");
                Ok(())
            }
            None => panic!("Impl error handling"),
        }
    }
}
