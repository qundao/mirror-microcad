// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Command to format a document.

use crate::document;

/// Format parameters
pub type FormatParameters = microcad_lang_format::FormatConfig;

/// Format a document.
pub trait Format {
    fn format(&mut self, params: &FormatParameters) -> document::Result<bool>;
}
