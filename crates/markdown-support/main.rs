// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate microcad_markdown_support;

use microcad_markdown_support::*;

fn main() -> std::io::Result<()> {
    let book_writer = book::BookWriter::new("./mdbook_test");

    let builtin = microcad_builtin::builtin_module();
    book_writer.write(&builtin)
}
