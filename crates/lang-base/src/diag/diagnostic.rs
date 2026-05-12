// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{GetSourceLocInfoByHash, SourceLocInfo};
use crate::{diag::*, src_ref::*};
use miette::SourceCode;

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
            Diagnostic::Trace(r)
            | Diagnostic::Info(r)
            | Diagnostic::Warning(r)
            | Diagnostic::Error(r) => r.to_string(),
        }
    }

    fn report(&self) -> &Report {
        match self {
            Diagnostic::Trace(r)
            | Diagnostic::Info(r)
            | Diagnostic::Warning(r)
            | Diagnostic::Error(r) => &r.value,
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
        source_by_hash: &impl GetSourceLocInfoByHash,
        options: &DiagRenderOptions,
    ) -> std::fmt::Result {
        let src_ref = self.src_ref();
        let hash = src_ref.source_hash();

        match &src_ref.is_none() {
            true => writeln!(f, "{}: {}", self.level(), self.message())?,
            false => {
                let source = match source_by_hash.get_source_loc_info_by_hash(hash) {
                    Some(source) => SourceLocInfo {
                        code: source.code,
                        url: source.url,
                        line_offset: source.line_offset,
                    },
                    None => SourceLocInfo::invalid(),
                };
                let wrapper = DiagnosticWrapper {
                    diagnostic: self,
                    source,
                };
                let handler = miette::GraphicalReportHandler::new_themed(options.theme());
                handler.render_report(&mut f, &wrapper)?
            }
        }

        Ok(())
    }

    /// Pretty print the diagnostics to a string, see `pretty_print` for more information
    pub fn to_pretty_string(
        &self,
        source_by_hash: &impl GetSourceLocInfoByHash,
        options: &DiagRenderOptions,
    ) -> String {
        let mut buff = String::new();
        self.pretty_print(&mut buff, source_by_hash, options)
            .expect("format to string can't fail");
        buff
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
    source: SourceLocInfo<'a>,
}

impl std::fmt::Debug for DiagnosticWrapper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for DiagnosticWrapper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let src_ref = self.diagnostic.src_ref();
        match self.diagnostic {
            Diagnostic::Trace(message) => write!(f, "trace: {src_ref}: {message}"),
            Diagnostic::Info(message) => write!(f, "info: {src_ref}: {message}"),
            Diagnostic::Warning(error) => write!(f, "warning: {src_ref}: {error}"),
            Diagnostic::Error(error) => write!(f, "error: {src_ref}: {error}"),
        }
    }
}

impl std::error::Error for DiagnosticWrapper<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.diagnostic.report().source()
    }
}

impl miette::Diagnostic for DiagnosticWrapper<'_> {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.diagnostic.report().code()
    }

    fn severity(&self) -> Option<miette::Severity> {
        match self.diagnostic {
            Diagnostic::Trace(_) => None,
            Diagnostic::Info(_) => Some(miette::Severity::Advice),
            Diagnostic::Warning(_) => Some(miette::Severity::Warning),
            Diagnostic::Error(_) => Some(miette::Severity::Error),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.diagnostic.report().help()
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.source)
    }

    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        self.diagnostic.report().diagnostic_source()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        self.diagnostic.report().labels().or_else(|| {
            let span = self.diagnostic.src_ref().as_miette_span()?;
            let label = miette::LabeledSpan::new_with_span(Some(self.diagnostic.message()), span);
            Some(Box::new(std::iter::once(label)))
        })
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn miette::Diagnostic> + 'a>> {
        self.diagnostic.report().related()
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.diagnostic.report().url()
    }
}
