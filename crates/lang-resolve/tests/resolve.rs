// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use test_that::prelude::*;

use microcad_lang_base::Artifact;

#[test]
fn resolve_inline_module_def() {
    let source = microcad_lang_base::Source::load("tests/test_cases/inline_module.µcad")
        .expect("No errors loading source");

    let ron = std::fs::read_to_string("tests/test_cases/inline_module.µcad.ir")
        .expect("No errors reading IR");
    let ir = microcad_lang_lower::Ir::from_ron(ron.as_str()).expect("No error deserializing IR");
    //let rst = microcad_lang_resolve::resolve(&source, &ir);
}
