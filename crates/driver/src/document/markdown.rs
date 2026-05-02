// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_lang_base::{Diagnostics, HashMap};
use miette::Diagnostic;
use thiserror::Error;
use url::Url;

use crate::document;
use microcad_lang_markdown::{Markdown, MarkdownError};

#[derive(Error, Debug, Diagnostic)]
pub enum MarkdownItemError {
    /// Error loading mdbook toml from file.
    #[error("Mdbook is not local: {0}")]
    NoLocalMarkdown(Url),

    /// Mdbook is not loaded
    #[error("Mdbook is not loaded")]
    NotLoaded,

    /// Error when loading mdbook.
    #[error("Markdown error: {0}")]
    MarkdownError(#[from] MarkdownError),
}

#[derive(Default)]
pub enum State {
    #[default]
    Raw,
    Loaded {
        markdown: Markdown,
    },
    Processed {
        markdown: Markdown,
        code_blocks: HashMap<String, Rc<document::SourceItem>>,
    },
}

impl document::MarkdownItem {
    pub fn load_from_file(&self) -> document::DiagResult {
        self.transition(|_| match self.file_path() {
            Some(path) => {
                let markdown = Markdown::load(path).map_err(|err| {
                    Diagnostics::single_error(MarkdownItemError::MarkdownError(err))
                })?;
                Ok(State::Loaded { markdown })
            }
            None => Err(Diagnostics::single_error(
                MarkdownItemError::NoLocalMarkdown(self.url.clone()),
            )),
        })
    }

    pub fn format(&'_ self) -> document::DiagResult<'_, bool> {
        self.load_from_file()?;
        let mut formatted = false;

        self.transition(|state| {
            let mut diags = Diagnostics::default();

            if let State::Loaded { mut markdown } = state {
                markdown
                    .code_blocks_mut()
                    .filter(|code_block| code_block.can_format())
                    .for_each(|code_block| {
                        let config = microcad_lang_format::FormatConfig::from(&self.config.format);
                        match microcad_lang_format::format_str(&code_block.code, &config) {
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
                    Ok(State::Loaded { markdown })
                }
            } else {
                Err(Diagnostics::single_error(MarkdownItemError::NotLoaded))
            }
        })?;

        Ok(formatted)
    }

    pub fn sync(&'_ self) -> document::DiagResult<'_> {
        Ok(match &*self.state.borrow() {
            State::Raw => (),
            State::Loaded { markdown } | State::Processed { markdown, .. } => markdown
                .save(self.file_path().expect("File path"))
                .expect("Error handling"),
        })
    }
}
