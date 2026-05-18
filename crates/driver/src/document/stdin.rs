// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use miette::IntoDiagnostic;

use crate::prelude as mu;

pub struct Stdin;

impl mu::Format for Stdin {
    fn format(&mut self, params: &mu::FormatParameters) -> mu::Result<bool> {
        let mut full_buffer = String::new();

        use std::io::Read;
        // Read all contents from stdin until EOF is reached
        std::io::stdin()
            .read_to_string(&mut full_buffer)
            .into_diagnostic()?;

        microcad_lang_format::format_str(&full_buffer, &params)
            .map(|s| {
                println!("{s}");
                s != full_buffer
            })
            .map_err(|err| miette::miette!("{err:?}"))
    }
}
