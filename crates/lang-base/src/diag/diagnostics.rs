// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::slice::Iter;

use crate::{GetSourceStrByHash, diag::*};

/// µcad source diagnostics.
#[derive(Debug, Default)]
pub struct Diagnostics {
    /// The number of overall errors in the evaluation process.
    pub error_count: u32,
    /// The number of overall warnings in the evaluation process.
    pub warning_count: u32,
    /// The list of diagnostics
    diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn single_error(report: impl Into<Report>) -> Self {
        Self {
            error_count: 1,
            warning_count: 0,
            diagnostics: vec![Diagnostic::Error(Refer::none(report.into()))],
        }
    }

    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    pub fn iter(&'_ self) -> Iter<'_, Diagnostic> {
        self.diagnostics.iter()
    }

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

    /// Merges another Diagnostics collection into this one.
    pub fn append(&mut self, mut other: Diagnostics) {
        self.error_count += other.error_count;
        self.warning_count += other.warning_count;
        self.diagnostics.append(&mut other.diagnostics);
    }
}

impl FromIterator<Diagnostics> for Diagnostics {
    fn from_iter<I: IntoIterator<Item = Diagnostics>>(iter: I) -> Self {
        let mut root = Diagnostics::default();
        for other in iter {
            root.append(other);
        }
        root
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
