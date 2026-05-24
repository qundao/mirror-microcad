// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use tower_lsp::lsp_types as lsp;

use microcad_driver::prelude as mu;
use mu::traits::*;

pub trait ToLsp {
    type Output;

    fn to_lsp(&self) -> Self::Output;
}

impl ToLsp for mu::SrcRef {
    type Output = Option<lsp::Range>;

    fn to_lsp(&self) -> Self::Output {
        match self.is_some() {
            true => {
                let start = lsp::Position::new(self.at.line, self.at.col - 1);
                let end =
                    lsp::Position::new(self.at.line, (self.at.col + self.range.len() as u32) - 1);

                Some(lsp::Range::new(start, end))
            }
            false => None,
        }
    }
}

impl ToLsp for mu::base::DiagLevel {
    type Output = lsp::DiagnosticSeverity;

    fn to_lsp(&self) -> Self::Output {
        use mu::base::DiagLevel::*;
        match &self {
            Trace => lsp::DiagnosticSeverity::HINT,
            Info => lsp::DiagnosticSeverity::INFORMATION,
            Warning => lsp::DiagnosticSeverity::WARNING,
            Error => lsp::DiagnosticSeverity::ERROR,
        }
    }
}

impl ToLsp for mu::Diagnostics {
    type Output = lsp::FullDocumentDiagnosticReport;

    fn to_lsp(&self) -> Self::Output {
        lsp::FullDocumentDiagnosticReport {
            result_id: None,
            items: self
                .iter()
                .filter_map(|diag| {
                    let message = diag.message();
                    match diag.src_ref().to_lsp() {
                        Some(range) => Some(lsp::Diagnostic::new(
                            range,
                            Some(diag.level().to_lsp()),
                            None,
                            None,
                            message,
                            None,
                            None,
                        )),
                        None => None,
                    }
                })
                .collect(),
        }
    }
}
