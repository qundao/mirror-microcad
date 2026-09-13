// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Issues are pieces of information that record what happened during execution.
//!
//! Unlike fatal errors returned via `Result<T, E>`, an `Issue` is non-blocking.
//! It captures domain occurrences (errors, warnings, informational messages)
//! and accumulates them inside an [`IssueList`] without interrupting the
//! primary evaluation flow.

use miette::Severity;

/// A trait to define an issue.
pub trait Issue: From<Self::Err> + From<Self::Warn> + From<Self::Info> {
    type Err;
    type Warn;
    type Info;

    fn severity(&self) -> Severity;
}

#[derive(Debug)]
pub struct IssueList<I: Issue> {
    issues: Vec<I>,
    err_count: usize,
    warn_count: usize,
}

impl<I: Issue> Default for IssueList<I> {
    fn default() -> Self {
        Self {
            issues: vec![],
            err_count: 0,
            warn_count: 0,
        }
    }
}

pub trait PushIssue<I: Issue> {
    fn push_err(&mut self, err: impl Into<I::Err>);
    fn push_warn(&mut self, warn: impl Into<I::Warn>);
    fn push_info(&mut self, info: impl Into<I::Info>);

    /// Pushes the error and returns a default value for `T`, so compilation can continue.
    fn catch<T: Default>(&mut self, err: impl Into<I::Err>) -> Result<T, I::Err> {
        self.push_err(err);
        Ok(T::default())
    }
}

impl<I: Issue> PushIssue<I> for IssueList<I> {
    fn push_err(&mut self, err: impl Into<I::Err>) {
        self.err_count += 1;
        // Convert input -> I::Err -> I
        let err_type: I::Err = err.into();
        self.issues.push(I::from(err_type));
    }

    fn push_warn(&mut self, warn: impl Into<I::Warn>) {
        self.warn_count += 1;
        let warn_type: I::Warn = warn.into();
        self.issues.push(I::from(warn_type));
    }

    fn push_info(&mut self, info: impl Into<I::Info>) {
        let info_type: I::Info = info.into();
        self.issues.push(I::from(info_type));
    }
}
