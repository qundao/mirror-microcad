// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Library tests

use microcad_lang_resolve::{Library, ResolveContext, library::LibraryRoot};

#[test]
fn load_source_inline_module() {
    let mut library = Library::new(LibraryRoot::default());
    let mut context = ResolveContext::new();

    library
        .load_source("tests/test_cases/inline_module.µcad", &mut context)
        .expect("No error");

    insta::assert_snapshot!("load_source_inline_module", library)
}

#[test]
fn load_source_file_module() {
    let mut library = Library::new(LibraryRoot::default());
    let mut context = ResolveContext::new();

    library
        .load_source("tests/test_cases/file_module.µcad", &mut context)
        .expect("No error");

    insta::assert_snapshot!("load_source_file_module", library)
}
