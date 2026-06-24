// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Command to format a document.

use crate::Result;

/// Format parameters
pub type FormatParameters = microcad_lang_format::FormatConfig;

/// Format a document.
pub trait Format {
    /// Returns true if the text has been formatted.
    fn format(&mut self, params: &FormatParameters) -> Result<bool>;
}
