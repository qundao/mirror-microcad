// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod markdown_test;

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/md_test_book_tests.rs"));

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/md_test_book_language.rs"));

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/md_test_book_tutorials.rs"));

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/md_test_book_examples.rs"));
