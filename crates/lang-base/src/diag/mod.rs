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

pub trait PushDiag<E> {
    fn push_diag(&mut self, err: impl Into<E>);

    /// Append diagnostics
    fn append_diags<T>(&mut self, errors: impl IntoIterator<Item = T>)
    where
        T: Into<E>,
    {
        for err in errors {
            self.push_diag(err);
        }
    }

    /// Push the error if the result is an error or return default.
    fn capture<T: Default>(&mut self, result: Result<T, impl Into<E>>) -> T {
        match result {
            Ok(t) => t,
            Err(err) => {
                self.push_diag(err);
                T::default()
            }
        }
    }

    /// Pushes the error and returns default value, so compilation can continue.
    fn catch<T: Default>(&mut self, err: impl Into<E>) -> Result<T, Box<E>> {
        self.push_diag(err);
        Ok(T::default())
    }

    fn err<T>(&mut self, err: impl Into<E>) -> Result<T, E> {
        Err(err.into())
    }
}
