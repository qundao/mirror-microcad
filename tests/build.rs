// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad pest test

/// pest test main
fn main() {
    microcad_pest_test::generate(
        "microcad_lang::parser::Parser",
        "microcad_lang::parser::Rule",
        "../lang/grammar.pest",
    );

    if let Err(err) = microcad_markdown_test::generate(
        "../books/tests/src",
        "md_test_book_tests.rs",
        "../books/tests/src/test_list.md",
    ) {
        panic!("error generating rust test code from markdown file: {err}");
    }

    if let Err(err) = microcad_markdown_test::generate(
        "../books/language/src",
        "md_test_book_language.rs",
        "../books/language/src/appendix/test_list.md",
    ) {
        panic!("error generating rust test code from markdown file: {err}");
    }

    if let Err(err) = microcad_markdown_test::generate(
        "../books/tutorials/src",
        "md_test_book_tutorials.rs",
        "../books/tutorials/src/test_list.md",
    ) {
        panic!("error generating rust test code from markdown file: {err}");
    }
}
