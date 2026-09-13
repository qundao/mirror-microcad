// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Diagnostics are issues that are presented to user.

mod diagnostic;
mod diagnostics;
mod issue;

pub use diagnostic::Diagnostic;
pub use diagnostics::{DiagRenderOptions, Diagnostics};
pub use issue::{Issue, IssueList, PushIssue};

pub use miette::Report;

/// Trait to implement `.into_diagnostics()`.
pub trait IntoDiagnostics {
    fn into_diagnostics(&self) -> Diagnostics;
}

impl<I: Issue + Clone + Into<miette::Report>> IntoDiagnostics for IssueList<I> {
    fn into_diagnostics(&self) -> Diagnostics {
        let mut diags = Diagnostics::new();
        self.iter().for_each(|issue| {
            diags.push(issue.clone());
        });
        diags
    }
}
