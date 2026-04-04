// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Markdown section

use crate::Paragraph;

/// A markdown section with heading and paragraphs.
#[derive(Debug, Clone, Default)]
pub struct Section {
    /// A heading, displayed with a `#` prefix in markdown.
    pub heading: String,
    /// Section level.
    pub level: i64,
    /// The section content, consisting of paragraphs.
    pub content: Vec<Paragraph>,
}

impl Section {
    /// A section with `n` levels deeper.
    pub fn nested(&self, n: i64) -> Section {
        Section {
            heading: self.heading.clone(),
            level: self.level + n,
            content: self.content.clone(),
        }
    }

    /// Returns `false` if this section has a heading and a content.
    pub fn is_empty(&self) -> bool {
        self.heading.is_empty() && self.content.is_empty()
    }
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "#".repeat(self.level as usize), self.heading)?;
        self.content
            .iter()
            .try_for_each(|line| writeln!(f, "{line}"))
    }
}
