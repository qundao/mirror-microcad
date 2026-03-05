// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a single markdown file for symbol.

use derive_more::{Deref, DerefMut};

use crate::md::Section;

/// Markdown struct, represented as a linear list of sections.
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Markdown(Vec<Section>);

impl Markdown {
    pub fn new(s: &str) -> Self {
        let mut sections = Vec::new();
        // Start with a default section (Level 1) for any leading text
        let mut current_section = Section {
            level: 0,
            ..Default::default()
        };
        let mut in_code_block = false;

        for line in s.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
            }

            let heading_info = if !in_code_block {
                Self::parse_heading(trimmed)
            } else {
                None
            };

            if let Some((level, title)) = heading_info {
                // If the current section has content or a title, save it
                if !current_section.is_empty() {
                    sections.push(current_section);
                }

                current_section = Section {
                    heading: title,
                    level,
                    content: Vec::new(),
                };
            } else {
                // Handle text before any heading exists
                current_section.content.push(line.to_string());
            }
        }

        if !current_section.is_empty() {
            sections.push(current_section);
        }

        Self(sections)
    }

    fn parse_heading(line: &str) -> Option<(i64, String)> {
        let count = line.chars().take_while(|c| *c == '#').count();

        if count > 0 {
            // Find the byte index where '#' ends
            let byte_idx = line
                .char_indices()
                .nth(count)
                .map(|(i, _)| i)
                .unwrap_or(line.len());
            let rest = &line[byte_idx..];

            // Standard Markdown: '#' must be followed by a space or be the end of line
            if rest.is_empty() || rest.starts_with(' ') {
                return Some((count as i64, rest.trim().to_string()));
            }
        }
        None
    }

    /// Add a new section.
    pub fn add_section(&mut self, section: Section) {
        self.0.push(section)
    }

    /// Nest another markdown
    pub fn nest(&mut self, md: Markdown, n: i64) {
        self.0.extend(md.0.into_iter().map(|s| s.nested(n)));
    }

    /// Write markdown to file.
    pub fn write(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.to_string().as_bytes())
    }
}

impl std::fmt::Display for Markdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, section) in self.0.iter().enumerate() {
            // Add a newline between sections for readability,
            // but not before the very first one.
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", section)?;
        }
        Ok(())
    }
}

#[test]
fn test_heading_parsing_and_display() {
    let input = "# Top\nContent\n## Sub\nMore content";
    let md = Markdown::new(input);

    assert_eq!(md.0.len(), 2);
    assert_eq!(md.0[0].level, 1);
    assert_eq!(md.0[0].heading, "Top");
    println!("{:#?}", md.0[0].content);

    assert!(!md.0[0].content.contains(&"Content\n".to_string()));
    assert_eq!(md.0[1].level, 2);
    assert_eq!(md.0[1].heading, "Sub");
    assert!(!md.0[0].content.contains(&"More content".to_string()));

    // Verify formatting includes the double newline you added in Display
    let output = md.0[0].to_string();
    assert!(output.starts_with("# Top\n\n"));
}

#[test]
fn test_unicode_heading() {
    let input = "### 🦀 Rust Section";
    let res = Markdown::parse_heading(input);
    assert_eq!(res, Some((3, "🦀 Rust Section".to_string())));
}
