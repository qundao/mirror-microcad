// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Markdown section

/// A markdown section with heading and lines.
#[derive(Debug, Clone, Default)]
pub struct Section {
    /// A heading, displayed with a `#` prefix in markdown.
    pub heading: String,
    /// Section level.
    pub level: i64,
    /// The section content
    pub content: Vec<String>,
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
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 1. Render the heading if it exists
        let hashes = "#".repeat(self.level as usize);
        writeln!(f, "{} {}\n", hashes, self.heading)?;

        // 2. Render the content lines
        for line in &self.content {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}
