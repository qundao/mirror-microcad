// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, ResourceLocation, Url};
use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

use crate::{
    Result, commands,
    document::{self, CaptureDiags, TryFilePath},
};
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

pub struct MarkdownDocument {
    url: Url,
    markdown: Option<Markdown>,
    diagnostics: Diagnostics,
}

impl MarkdownDocument {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            markdown: None,
            diagnostics: Default::default(),
        }
    }
}

impl ResourceLocation for MarkdownDocument {
    fn url(&self) -> &Url {
        &self.url
    }
}

impl TryFilePath for MarkdownDocument {}

impl CaptureDiags for MarkdownDocument {
    fn diags(&self) -> &Diagnostics {
        &self.diagnostics
    }

    fn diags_mut(&mut self) -> &mut Diagnostics {
        &mut self.diagnostics
    }
}

impl commands::LoadFromFile for document::Markdown {
    fn load_from_file(&mut self) -> Result {
        self.markdown = Some(Markdown::load(self.try_file_path()?).into_diagnostic()?);
        Ok(())
    }
}

impl commands::Format for document::Markdown {
    fn format(&mut self, params: &commands::FormatParameters) -> Result<bool> {
        let mut formatted = false;
        let config = params;
        self.diagnostics.clear();

        match &mut self.markdown {
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
                                self.diagnostics.append(diag);
                            }
                        }
                    });

                if self.diagnostics.has_errors() {
                    Err(miette::miette!("Error formatting markdown"))
                } else {
                    Ok(formatted)
                }
            }
            None => Err(MarkdownItemError::NotLoaded.into()),
        }
    }
}

impl commands::Sync for document::Markdown {
    fn sync(&self) -> Result {
        match &self.markdown {
            Some(markdown) => markdown.save(self.try_file_path()?).into_diagnostic(),
            None => Err(MarkdownItemError::NotLoaded.into()),
        }
    }
}
