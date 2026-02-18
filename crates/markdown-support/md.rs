// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Microcad micro markdown parser and writer

/// A markdown section with heading and lines.
#[derive(Debug, Clone, Default)]
pub struct Section {
    /// A heading, will have a `#` if level > 0
    pub heading: String,
    /// Section level.
    pub level: i64,
    /// The section content
    pub content: Vec<String>,
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 1. Render the heading if it exists
        if self.level > 0 {
            let hashes = "#".repeat(self.level as usize);
            writeln!(f, "{} {}\n", hashes, self.heading)?;
        } else if !self.heading.is_empty() {
            // Fallback for level 0 sections that might have a title
            writeln!(f, "{}\n", self.heading)?;
        }

        // 2. Render the content lines
        for line in &self.content {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Markdown(Vec<Section>);

impl Markdown {
    pub fn new(s: &str) -> Markdown {
        let mut sections = Vec::new();
        let mut current_section = Section::default();
        let mut in_code_block = false;

        for line in s.lines() {
            let trimmed = line.trim();

            // Toggle code block state
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
            }

            // Check for heading if not in a code block
            let heading_info = if !in_code_block {
                Self::parse_heading(line)
            } else {
                None
            };

            if let Some((level, title)) = heading_info {
                // If the current section has data, push it before starting a new one
                if current_section.level > 0
                    || !current_section.content.is_empty()
                    || !current_section.heading.is_empty()
                {
                    sections.push(current_section);
                }

                current_section = Section {
                    heading: title,
                    level,
                    content: Vec::new(),
                };
            } else {
                // Otherwise, append line to content of current section
                current_section.content.push(line.to_string());
            }
        }

        // Push the final section
        if current_section.level > 0
            || !current_section.content.is_empty()
            || !current_section.heading.is_empty()
        {
            sections.push(current_section);
        }

        Markdown(sections)
    }

    /// Helper to identify "# Heading" and return (level, title)
    fn parse_heading(line: &str) -> Option<(i64, String)> {
        let count = line.chars().take_while(|c| *c == '#').count();
        if count > 0 {
            let rest = &line[count..];
            if rest.starts_with(' ') {
                return Some((count as i64, rest.trim().to_string()));
            }
        }
        None
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
