// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Library tests

use microcad_lang_base::{DiagRenderOptions, Diagnostics};
use microcad_lang_resolve::{Library, ResolveContext};

fn ctx() -> ResolveContext {
    let mut ctx = ResolveContext::new();
    ctx.load_external_library("../../crates/std/lib/std");
    ctx
}

#[test]
fn load_source_inline_module() {
    let context = ctx();
    match ctx().load_file("tests/test_cases/inline_module.µcad") {
        Ok(library) => insta::assert_snapshot!("load_source_inline_module", library),
        Err(err) => {
            let diagnostics = Diagnostics::from(context.diag);

            panic!(
                "{diagnostics}\n{err:?}",
                diagnostics = diagnostics
                    .render_to_string(&ResolveContext::new(), &DiagRenderOptions::default())
                    .unwrap()
            )
        }
    }
}

#[test]
fn load_source_file_module() {
    let library = ctx()
        .load_file("tests/test_cases/file_module.µcad")
        .expect("No error");

    insta::assert_snapshot!("load_source_file_module", library)
}
