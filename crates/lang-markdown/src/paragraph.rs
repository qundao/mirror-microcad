// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::code_block::CodeBlock;

/// A paragraph. Each parameter ends with a new line.
#[derive(Debug, Clone, PartialEq)]
pub enum Paragraph {
    /// A paragraph with text.
    Text(String),

    /// A µcad code block starting with ```µcad or with `[![test](...)` banner.
    CodeBlock(CodeBlock),

    /// A table. Each line starts with `|`.
    Table(String),
}

impl std::fmt::Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Paragraph::Text(text) => writeln!(f, "{text}"),
            Paragraph::CodeBlock(code_block) => writeln!(f, "{code_block}"),
            Paragraph::Table(table) => writeln!(f, "{table}"),
        }
    }
}
