// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Microcad micro markdown parser and writer

/// A markdown section with heading and examples.
#[derive(Debug, Clone, Default)]
pub struct Section {
    pub heading: String,
    pub level: i64,
    pub lines: Vec<String>,
    pub children: Vec<Section>,
}

impl Section {
    pub fn add_sub_section(&mut self, mut section: Section) -> &mut Section {
        section.level = self.level + 1;
        self.children.push(section);

        self.children.last_mut().expect("No error")
    }

    /// Parse from markdown into a single root Section
    pub fn from_markdown(markdown: &str) -> Section {
        let mut stack: Vec<Section> = Vec::new();
        let mut in_code_block = false;

        // Synthetic root node
        stack.push(Section {
            heading: String::new(),
            level: 0,
            lines: Vec::new(),
            children: Vec::new(),
        });

        let mut lines = markdown.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();

            // Track fenced code blocks
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
            }

            if !in_code_block {
                // 1️⃣ Standard # heading
                if let Some((level, heading)) = Self::parse_hash_heading(trimmed) {
                    Self::start_section(level, heading, &mut stack);
                    continue;
                }

                // 2️⃣ Implicit heading
                if Self::is_implicit_heading(trimmed, lines.peek()) {
                    Self::start_section(1, trimmed.to_string(), &mut stack);
                    continue;
                }
            }

            // Normal content line
            if let Some(current) = stack.last_mut() {
                current.lines.push(line.to_string());
            }
        }

        // Collapse stack into tree
        while stack.len() > 1 {
            let section = stack.pop().unwrap();
            let parent = stack.last_mut().unwrap();
            parent.children.push(section);
        }

        stack.pop().unwrap()
    }
    fn parse_hash_heading(line: &str) -> Option<(i64, String)> {
        let count = line.chars().take_while(|c| *c == '#').count();
        if count > 0 && line.chars().nth(count) == Some(' ') {
            Some((count as i64, line[count + 1..].to_string()))
        } else {
            None
        }
    }

    fn is_implicit_heading(line: &str, next: Option<&&str>) -> bool {
        if line.is_empty() {
            return false;
        }

        // Heuristic:
        // - not a code fence
        // - short line
        // - next line is blank or exists
        let short = line.len() < 80;
        let next_blank = next.map(|l| l.trim().is_empty()).unwrap_or(true);

        short && next_blank
    }

    fn start_section(level: i64, heading: String, stack: &mut Vec<Section>) {
        while let Some(top) = stack.last() {
            if top.level >= level {
                let completed = stack.pop().unwrap();
                let parent = stack.last_mut().unwrap();
                parent.children.push(completed);
            } else {
                break;
            }
        }

        stack.push(Section {
            heading,
            level,
            lines: Vec::new(),
            children: Vec::new(),
        });
    }

    fn fmt_with_spacing(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        top_level: bool,
    ) -> std::fmt::Result {
        // Add spacing between sibling sections (but not before first root)
        if !top_level {
            writeln!(f)?;
        }

        // Write heading
        if self.level > 0 {
            let hashes = "#".repeat(self.level as usize);
            writeln!(f, "{} {}", hashes, self.heading)?;
            writeln!(f)?;
        }

        // Write section lines exactly as stored
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }

        // Recursively render children
        for child in &self.children {
            child.fmt_with_spacing(f, false)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_spacing(f, true)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Markdown(pub Section);

impl Markdown {
    pub fn new(s: &str) -> Markdown {
        Self(Section::from_markdown(s))
    }
}

impl std::fmt::Display for Markdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt_with_spacing(f, true)
    }
}
