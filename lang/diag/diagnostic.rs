// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;
use std::iter::once;
use miette::{GraphicalReportHandler, LabeledSpan, Report, Severity, SourceCode};
use crate::{diag::*, resolve::*, src_ref::*};
use crate::syntax::MietteSourceFile;

/// Diagnostic message with source code reference attached.
pub enum Diagnostic {
    /// Trace message.
    Trace(Refer<Report>),
    /// Informative message.
    Info(Refer<Report>),
    /// Warning.
    Warning(Refer<Report>),
    /// Error.
    Error(Refer<Report>),
}

impl Diagnostic {
    /// Get diagnostic level.
    pub fn level(&self) -> Level {
        match self {
            Diagnostic::Trace(_) => Level::Trace,
            Diagnostic::Info(_) => Level::Info,
            Diagnostic::Warning(_) => Level::Warning,
            Diagnostic::Error(_) => Level::Error,
        }
    }

    /// Get message (errors will be serialized).
    pub fn message(&self) -> String {
        match self {
            Diagnostic::Trace(msg) | Diagnostic::Info(msg) => msg.to_string(),
            Diagnostic::Warning(err) | Diagnostic::Error(err) => err.to_string(),
        }
    }

    /// Return line of the error
    pub fn line(&self) -> Option<usize> {
        let src_ref = match self {
            Diagnostic::Trace(r) => r.src_ref(),
            Diagnostic::Info(r) => r.src_ref(),
            Diagnostic::Warning(r) => r.src_ref(),
            Diagnostic::Error(r) => r.src_ref(),
        };
        src_ref.as_ref().map(|r| r.at.line)
    }

    fn report(&self) -> &Report {
        match self {
            Diagnostic::Trace(r)
            | Diagnostic::Info(r)
            | Diagnostic::Warning(r)
            | Diagnostic::Error(r) => &r.value
        }
    }

    /// Pretty print the diagnostic.
    ///
    /// This will print the diagnostic to the given writer, including the source code reference.
    ///
    /// # Arguments
    ///
    /// * `w` - The writer to write to.
    /// * `source_file_by_hash` - Hash provider to get the source file by hash.
    ///
    /// This will print:
    ///
    /// ```text
    /// error: This is an error
    ///   ---> filename:1:8
    ///     |
    ///  1  | part Circle(radius: length) {}
    ///     |        ^^^^^^
    /// ```
    pub fn pretty_print(
        &self,
        mut f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
        _line_offset: usize,
    ) -> std::fmt::Result {
        let src_ref = self.src_ref();

        let source_file = source_by_hash.get_by_hash(src_ref.source_hash());

        fn make_relative(path: &std::path::Path) -> String {
            let current_dir = std::env::current_dir().expect("current dir");
            if let Ok(path) = path.canonicalize() {
                pathdiff::diff_paths(path, current_dir)
                    .expect("related paths:\n  {path:?}\n  {current_dir:?}")
            } else {
                path.to_path_buf()
            }
            .to_string_lossy()
            .to_string()
        }

        match &src_ref {
            SrcRef(None) => writeln!(f, "{}: {}", self.level(), self.message())?,
            SrcRef(Some(_)) => {
                let miette_source = source_file
                    .as_ref()
                    .map(|s| s.miette_source(make_relative(&s.filename())))
                    .unwrap_or_else(|_| MietteSourceFile::invalid());
                let wrapper = DiagnosticWrapper {
                    diagnostic: &self,
                    source: miette_source,
                };
                let handler = GraphicalReportHandler::new();
                handler.render_report(&mut f, &wrapper)?
            }
        }

        Ok(())
    }
}

impl SrcReferrer for Diagnostic {
    fn src_ref(&self) -> SrcRef {
        match self {
            Diagnostic::Trace(message) => message.src_ref(),
            Diagnostic::Info(message) => message.src_ref(),
            Diagnostic::Warning(error) => error.src_ref(),
            Diagnostic::Error(error) => error.src_ref(),
        }
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Diagnostic::Trace(message) => write!(f, "trace: {}: {message}", self.src_ref()),
            Diagnostic::Info(message) => write!(f, "info: {}: {message}", self.src_ref()),
            Diagnostic::Warning(error) => write!(f, "warning: {}: {error}", self.src_ref()),
            Diagnostic::Error(error) => write!(f, "error: {}: {error}", self.src_ref()),
        }
    }
}

impl std::fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Diagnostic::Trace(message) => write!(f, "trace: {}: {message}", self.src_ref()),
            Diagnostic::Info(message) => write!(f, "info: {}: {message}", self.src_ref()),
            Diagnostic::Warning(error) => write!(f, "warning: {}: {error:?}", self.src_ref()),
            Diagnostic::Error(error) => write!(f, "error: {}: {error:?}", self.src_ref()),
        }
    }
}

struct DiagnosticWrapper<'a> {
    diagnostic: &'a Diagnostic,
    source: MietteSourceFile<'a>,
}

impl std::fmt::Debug for DiagnosticWrapper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.diagnostic)
    }
}

impl std::fmt::Display for DiagnosticWrapper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.diagnostic)
    }
}

impl std::error::Error for DiagnosticWrapper<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.diagnostic.report().source()
    }
}

impl miette::Diagnostic for DiagnosticWrapper<'_> {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.diagnostic.report().code()
    }

    fn severity(&self) -> Option<Severity> {
        match self.diagnostic {
            Diagnostic::Trace(_) => None,
            Diagnostic::Info(_) => Some(Severity::Advice),
            Diagnostic::Warning(_) => Some(Severity::Warning),
            Diagnostic::Error(_) => Some(Severity::Error),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.diagnostic.report().help()
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.source)
    }

    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        self.diagnostic.report().diagnostic_source()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item=LabeledSpan> + '_>> {
        self.diagnostic.report().labels()
            .or_else(|| {
                let span = self.diagnostic.src_ref().as_miette_span()?;
                let label = LabeledSpan::new_with_span(Some(self.diagnostic.to_string()), span);
                Some(Box::new(once(label)))
            })
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item=&'a dyn miette::Diagnostic> + 'a>> {
        self.diagnostic.report().related()
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.diagnostic.report().url()
    }
}