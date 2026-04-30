// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Handling of diagnostic errors.
//!
//! While *evaluation* µcad is collecting [`Diagnostic`] messages.
//!
//! This is done in [`DiagHandler`] by providing the following traits:
//!
//! - [`PushDiag`]: Collects error in [`DiagHandler`]
//! - [`Diag`]: Get diagnostic messages

mod diag_handler;
mod diagnostic;
mod diagnostics;
mod level;

pub use diag_handler::*;
pub use diagnostic::*;
pub use diagnostics::*;
pub use level::*;
use miette::{Diagnostic as MietteDiagnostic, Report};

use thiserror::Error;

use crate::src_ref::*;
use microcad_core::hash::HashSet;

/// Diagnostic error.
#[derive(Debug, Error, MietteDiagnostic)]
pub enum DiagError {
    /// Cannot continue evaluation after error limit has been reached.
    #[error("Error limit reached: Stopped evaluation after {0} errors")]
    ErrorLimitReached(u32),
}

/// Result type of any resolve.
pub type DiagResult<T> = std::result::Result<T, DiagError>;

/// A trait to add diagnostics with different levels conveniently.
pub trait PushDiag {
    /// Push a diagnostic message (must be implemented).
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()>;

    /// Push new trace message.
    fn trace(&mut self, src: &impl SrcReferrer, message: String) {
        self.push_diag(Diagnostic::Trace(Refer::new(
            Report::msg(message),
            src.src_ref(),
        )))
        .expect("could not push diagnostic trace message");
    }
    /// Push new informative message.
    fn info(&mut self, src: &impl SrcReferrer, message: String) {
        self.push_diag(Diagnostic::Info(Refer::new(
            Report::msg(message),
            src.src_ref(),
        )))
        .expect("could not push diagnostic info message");
    }
    /// Push new warning.
    fn warning(&mut self, src: &impl SrcReferrer, err: impl Into<Report>) -> DiagResult<()> {
        let err = Diagnostic::Warning(Refer::new(err.into(), src.src_ref()));
        if cfg!(feature = "ansi-color") {
            log::warn!("{}", color_print::cformat!("<y,s>{err}</>"));
        } else {
            log::warn!("{err}");
        }
        self.push_diag(err)
    }
    /// Push new error.
    fn error(&mut self, src: &impl SrcReferrer, err: impl Into<Report>) -> DiagResult<()> {
        let err = Diagnostic::Error(Refer::new(err.into(), src.src_ref()));
        if cfg!(feature = "ansi-color") {
            log::error!("{}", color_print::cformat!("<r,s>{err}</>"));
        } else {
            log::error!("{err}");
        }
        self.push_diag(err)
    }
}

/// Diagnosis trait gives access about collected errors.
pub trait Diag {
    /// Pretty print all errors.
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result;

    /// Pretty write all errors into a file.
    fn write_diagnosis(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "{}", self.diagnosis())
    }

    /// Get pretty printed errors as string.
    fn diagnosis(&self) -> String {
        let mut str = String::new();
        self.fmt_diagnosis(&mut str).expect("displayable diagnosis");
        str
    }

    /// Returns true if there are warnings.
    fn has_warnings(&self) -> bool {
        self.warning_count() > 0
    }

    /// Return number of occurred warnings.
    fn warning_count(&self) -> u32;

    /// Returns true if there are errors.
    fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    /// Return number of occurred errors.
    fn error_count(&self) -> u32;

    /// Return all lines with errors
    fn error_lines(&self) -> HashSet<usize>;

    /// Return all lines with warnings
    fn warning_lines(&self) -> HashSet<usize>;
}
