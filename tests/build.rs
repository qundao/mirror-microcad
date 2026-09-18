// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad markdown test

use miette::{IntoDiagnostic, miette};

fn update_book(name: &str) -> miette::Result<()> {
    match microcad_markdown_test::generate(
        format!("../books/{name}"),
        format!("md_test_book_{name}.rs"),
        format!("../books/{name}/src/test_list.md"),
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(miette!(
            "error generating rust test code from markdown book '{name}': {err}"
        )),
    }
}
