// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later
/// Pretty print diagnostics

pub type PrintDiagnosticsParameters = microcad_lang_base::DiagRenderOptions;

pub trait PrintDiagnostics {
    fn print_diagnostics(
        &self,
        f: &mut dyn std::fmt::Write,
        params: &PrintDiagnosticsParameters,
    ) -> std::fmt::Result;

    fn diagnostics_string(&self, params: &PrintDiagnosticsParameters) -> String {
        let mut buffer = String::new();
        match self.print_diagnostics(&mut buffer, params) {
            Ok(_) | Err(_) => buffer,
        }
    }
}
