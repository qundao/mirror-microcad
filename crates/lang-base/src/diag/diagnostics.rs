// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{GetSourceStrByHash, diag::*};
use derive_more::Deref;

/// µcad source diagnostics.
#[derive(Debug, Default, Deref)]
pub struct Diagnostics {
    /// The number of overall errors in the evaluation process.
    pub error_count: u32,
    /// The number of overall warnings in the evaluation process.
    pub warning_count: u32,
    /// The list of diagnostics
    #[deref]
    pub diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Clear diagnostics.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.error_count = 0;
        self.warning_count = 0;
    }

    /// Pretty print this list of diagnostics.
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceStrByHash,
        line_offset: usize,
        options: &DiagRenderOptions,
    ) -> std::fmt::Result {
        self.diagnostics
            .iter()
            .try_for_each(|diag| diag.pretty_print(f, source_by_hash, line_offset, options))
    }
}

impl PushDiag for Diagnostics {
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()> {
        match &diag {
            Diagnostic::Error(_) => {
                self.error_count += 1;
            }
            Diagnostic::Warning(_) => {
                self.warning_count += 1;
            }
            _ => (),
        }

        self.diagnostics.push(diag);
        Ok(())
    }
}
