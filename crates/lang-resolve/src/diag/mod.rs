// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod error;

pub use error::{ResolveError, ResolveErrorKind};
use microcad_lang_base::{Issue, SrcRef, SrcReferrer};
use microcad_lang_lower::{LowerInfo, LowerIssue, LowerWarning};
use microcad_lang_parse::{ParseInfo, ParseIssue, ParseWarning};
use miette::{Diagnostic, Severity};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ResolveWarning {
    #[error("{0}")]
    Parse(#[from] ParseWarning),
    #[error("{0}")]
    Lower(#[from] LowerWarning),
}

#[derive(Debug, Error, Diagnostic)]
pub enum ResolveInfo {
    #[error("{0}")]
    Parse(#[from] ParseInfo),
    #[error("{0}")]
    Lower(#[from] LowerInfo),
}

#[derive(Debug, Error, Diagnostic)]
pub enum ResolveIssue {
    /// A parse error.
    #[error(transparent)]
    #[diagnostic(severity(Error), code(resolve::error))]
    Err(#[from] ResolveError),

    /// A warning from the parser.
    #[error(transparent)]
    #[diagnostic(severity(Warning), code(resolve::warning))]
    Warn(#[from] ResolveWarning),

    /// An info from the parser.
    #[error(transparent)]
    #[diagnostic(severity(Advice), code(resolve::info))]
    Info(#[from] ResolveInfo),
}

impl Issue for ResolveIssue {
    type Err = ResolveError;
    type Warn = ResolveWarning;
    type Info = ResolveInfo;

    fn severity(&self) -> Severity {
        match self {
            Self::Err(_) => Severity::Error,
            Self::Warn(_) => Severity::Warning,
            Self::Info(_) => Severity::Advice,
        }
    }
}

impl From<ParseIssue> for ResolveIssue {
    fn from(issue: ParseIssue) -> Self {
        match issue {
            ParseIssue::Err(err) => ResolveIssue::Err(err.into()),
            ParseIssue::Warn(warn) => ResolveIssue::Warn(warn.into()),
            ParseIssue::Info(info) => ResolveIssue::Info(info.into()),
        }
    }
}

impl From<LowerIssue> for ResolveIssue {
    fn from(issue: LowerIssue) -> Self {
        match issue {
            LowerIssue::Err(err) => ResolveIssue::Err(err.into()),
            LowerIssue::Warn(warn) => ResolveIssue::Warn(warn.into()),
            LowerIssue::Info(info) => ResolveIssue::Info(info.into()),
        }
    }
}

pub type ResolveResult<T> = Result<T, ResolveError>;

impl SrcReferrer for ResolveIssue {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Err(err) => err.src_ref(),
            _ => SrcRef::none(),
        }
    }
}
