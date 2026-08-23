// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mdbook `book.toml` Configuraion file

use microcad_lang_markdown::WriteToFile;

pub struct Config;

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let book_toml: toml::Value =
            toml::de::from_str(include_str!("book.toml")).expect("Valid toml");
        let str = toml::ser::to_string(&book_toml).expect("No error");
        write!(
            f,
            r#"# Copyright © 2026 The µcad authors <info@microcad.xyz>
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# NOTE: Auto-generated code.
# This markdown book has been generated from µcad source via `microcad-docgen`.
# Changes in the book might be overwritten.
{str}
"#
        )
    }
}

impl WriteToFile for Config {}
