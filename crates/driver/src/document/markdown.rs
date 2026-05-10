// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_lang_base::{Diagnostics, HashMap, ResourceLocation};
use miette::Diagnostic;
use thiserror::Error;
use url::Url;

use crate::{commands, document};
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
        code_blocks: HashMap<String, Rc<document::SourceAsset>>,
    },
}

impl commands::LoadFromFile for document::MarkdownAsset {
    fn load_from_file(&self) -> document::Result {
        self.transition(|_| match self.to_file_path() {
            Some(path) => {
                let markdown = Markdown::load(path).map_err(MarkdownItemError::MarkdownError)?;
                Ok(State::Loaded { markdown })
            }
            None => Err(MarkdownItemError::NoLocalMarkdown(self.url.clone()).into()),
        })
    }
}

impl commands::Format for document::MarkdownAsset {
    fn format(&self, params: &commands::FormatParameters) -> document::Result<bool> {
        use crate::commands::LoadFromFile;
        self.load_from_file()?;
        let mut formatted = false;
        let config = params;

        self.transition(|state| {
            let mut diags = Diagnostics::default();

            if let State::Loaded { mut markdown } = state {
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
                    Err(diags)
                } else {
                    Ok(State::Loaded { markdown })
                }
            } else {
                Err(MarkdownItemError::NotLoaded.into())
            }
        })?;

        Ok(formatted)
    }
}

impl commands::Sync for document::MarkdownAsset {
    fn sync(&self) -> document::Result {
        match &*self.state.borrow() {
            State::Raw => (),
            State::Loaded { markdown } | State::Processed { markdown, .. } => markdown
                .save(self.to_file_path().expect("File path"))
                .expect("Error handling"),
        }
        Ok(())
    }
}
