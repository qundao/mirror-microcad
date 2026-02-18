// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::IsTerminal;
use crate::{diag::*, resolve::*};

/// Handler for diagnostics.
#[derive(Default)]
pub struct DiagHandler {
    /// The list of diagnostics per source file.
    pub diag_list: DiagList,
    /// The number of overall errors in the evaluation process.
    error_count: u32,
    /// The number of overall errors in the evaluation process.
    warning_count: u32,
    /// The maximum number of collected errors until abort
    /// (`0` means unlimited number of errors).
    error_limit: Option<u32>,
    /// `true` after the first time error limit was reached
    error_limit_reached: bool,
    /// Treat warnings as errors if `true`.
    warnings_as_errors: bool,
    /// Line offset for error and warning messages.
    line_offset: usize,
    /// Diagnostic rendering options
    pub render_options: DiagRenderOptions,
}

/// Options that control the rendering of diagnostics
#[derive(Debug)]
pub struct DiagRenderOptions {
    /// Render diagnostic with colors
    pub color: bool,
    /// Render diagnostic with unicode characters
    pub unicode: bool,
}

impl Default for DiagRenderOptions {
    fn default() -> Self {
        DiagRenderOptions {
            color: std::env::var("NO_COLOR").as_deref().unwrap_or("0") == "0",
            unicode: std::io::stdout().is_terminal() && std::io::stderr().is_terminal()
        }
    }
}

impl DiagRenderOptions {
    /// Get the miette theme for the options
    pub fn theme(&self) -> miette::GraphicalTheme {
        match (self.unicode, self.color) {
            (true, true) => miette::GraphicalTheme::unicode(),
            (true, false) => miette::GraphicalTheme::unicode_nocolor(),
            (false, true) => miette::GraphicalTheme::ascii(),
            (false, false) => miette::GraphicalTheme::none(),
        }
    }
}

/// Handler for diagnostics.
impl DiagHandler {
    /// Create new diag handler.
    pub fn new(line_offset: usize) -> Self {
        Self {
            line_offset,
            ..Default::default()
        }
    }

    /// Pretty print all errors of all files.
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::fmt::Result {
        self.diag_list
            .pretty_print(f, source_by_hash, self.line_offset, &self.render_options)
    }

    /// Return overall number of occurred errors.
    pub fn warning_count(&self) -> u32 {
        self.warning_count
    }

    /// Return overall number of occurred errors.
    pub fn error_count(&self) -> u32 {
        self.error_count
    }

    /// return lines with errors
    pub fn error_lines(&self) -> std::collections::HashSet<usize> {
        self.diag_list
            .iter()
            .filter_map(|d| {
                if d.level() == Level::Error {
                    d.line().map(|line| line + self.line_offset)
                } else {
                    None
                }
            })
            .collect()
    }

    /// return lines with warnings
    pub fn warning_lines(&self) -> std::collections::HashSet<usize> {
        self.diag_list
            .iter()
            .filter_map(|d| {
                if d.level() == Level::Warning {
                    d.line().map(|line| line + self.line_offset)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Clear all errors and warnings
    pub fn clear(&mut self) {
        self.diag_list.clear();
        self.error_count = 0;
        self.warning_count = 0;
    }
}

impl PushDiag for DiagHandler {
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()> {
        if let Some(error_limit) = self.error_limit {
            if self.error_count >= error_limit && !self.error_limit_reached {
                self.error(
                    &SrcRef(None),
                    DiagError::ErrorLimitReached(error_limit),
                )?;
                self.error_limit_reached = true;
            }
            return Err(DiagError::ErrorLimitReached(error_limit));
        }

        match &diag {
            Diagnostic::Error(_) => {
                self.error_count += 1;
            }
            Diagnostic::Warning(_) => {
                if self.warnings_as_errors {
                    self.error_count += 1;
                } else {
                    self.warning_count += 1;
                }
            }
            _ => (),
        }

        self.diag_list.push_diag(diag)
    }
}
