// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Format Specification syntax element

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

/// Format specification.
#[derive(Clone, Debug, Default, PartialEq, SrcReferrer)]
pub struct FormatSpec {
    /// Precision for number formatting.
    pub precision: Option<u32>,
    /// Alignment width (leading zeros).
    pub width: Option<u32>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for FormatSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.width, self.precision) {
            (Some(width), Some(precision)) => write!(f, "0{width}.{precision}"),
            (None, Some(precision)) => write!(f, ".{precision}"),
            (Some(width), None) => write!(f, "0{width}"),
            _ => Ok(()),
        }
    }
}
