// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FormatError {
    /// Parse errors.
    #[error("Parse errors: {0:?}")]
    ParseErrors(Vec<microcad_syntax::ParseError>),

    /// Error formatting a markdown code block
    #[error("Error formatting code block `{name}`: \n{error}")]
    CodeBlock {
        name: String,
        error: Box<FormatError>,
    },

    /// Error when processing mdbook
    #[error("Error processing mdbook")]
    MdBookDirectoryError(#[from] microcad_lang_markdown::MdBookDirectoryError),

    /// Error formatting a markdown code block
    #[error("Error formatting mdbook in {src_path:?}: {errors:#?}")]
    MdBook {
        src_path: std::path::PathBuf,
        errors: HashMap<std::path::PathBuf, Vec<FormatError>>,
    },
}
