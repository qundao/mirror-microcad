// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a single markdown file for symbol.

use std::str::FromStr;

use derive_more::{Deref, DerefMut};
use thiserror::Error;

use crate::{CodeBlock, Paragraph, Section, code_block::CodeBlockHeader};

#[derive(Error, Debug)]
pub enum MarkdownError {
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unexpected end of file while parsing")]
    UnexpectedEOF,

    #[error("Malformed code block header: {0}")]
    MalformedCodeBlock(String),

    #[error("Invalid test result: {0}")]
    InvalidTestResult(String),

    #[error("Malformed header")]
    MalformedHeader,

    #[error("Missing fence")]
    MissingFence,
}

/// Markdown struct, represented as a linear list of sections.
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Markdown(Vec<Section>);

impl std::str::FromStr for Markdown {
    type Err = MarkdownError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut sections = Vec::new();
        let mut current_section = Section::default();
        let mut lines = input.lines().enumerate().peekable();

        while let Some((_, line)) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // 1. Headings
            if trimmed.starts_with('#') {
                if !current_section.heading.is_empty() || !current_section.content.is_empty() {
                    sections.push(current_section);
                }

                let level = trimmed.chars().take_while(|&c| c == '#').count() as i64;
                assert!(level > 0);

                current_section = Section {
                    heading: trimmed.trim_start_matches('#').trim().to_string(),
                    level,
                    content: Vec::new(),
                };
            }
            // 2. Code Blocks
            else if CodeBlockHeader::is_code_block_start(line) {
                let block = CodeBlock::parse(&mut lines)?;
                current_section.content.push(Paragraph::CodeBlock(block));
            }
            // 3. Tables
            else if trimmed.starts_with('|') {
                current_section
                    .content
                    .push(Paragraph::Table(line.to_string()));
            }
            // 4. Text
            else {
                current_section
                    .content
                    .push(Paragraph::Text(line.to_string()));
            }
        }

        sections.push(current_section);
        Ok(Self(sections))
    }
}

impl Markdown {
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, MarkdownError> {
        let input = std::fs::read_to_string(path)?;
        Markdown::from_str(&input)
    }

    pub fn update(path: impl AsRef<std::path::Path>) -> Result<Self, MarkdownError> {
        let md = Markdown::load(&path)?;
        md.write(path)?;
        Ok(md)
    }
    /// Write markdown to file.
    pub fn write(&self, path: impl AsRef<std::path::Path>) -> Result<(), MarkdownError> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        Ok(file.write_all(self.to_string().as_bytes())?)
    }

    /// Add a new section.
    pub fn add_section(&mut self, section: Section) {
        self.0.push(section)
    }

    /// Nest another markdown
    pub fn nest(&mut self, md: Markdown, n: i64) {
        self.0.extend(md.0.into_iter().map(|s| s.nested(n)));
    }

    /// Returns an iterator over all code blocks in the entire document.
    pub fn code_blocks(&self) -> impl Iterator<Item = &CodeBlock> {
        self.0
            .iter() // Iterate over Vec<Section>
            .flat_map(|section| section.content.iter()) // Flatten Paragraphs
            .filter_map(|paragraph| {
                if let Paragraph::CodeBlock(block) = paragraph {
                    Some(block)
                } else {
                    None
                }
            })
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
    let md = Markdown::from_str(input).unwrap();

    assert_eq!(md.0.len(), 2);
    assert_eq!(md.0[0].level, 1);
    assert_eq!(md.0[0].heading, "Top");
    println!("{:#?}", md.0[0].content);

    assert!(
        md.0[0]
            .content
            .contains(&Paragraph::Text("Content".to_string()))
    );
    assert_eq!(md.0[1].level, 2);
    assert_eq!(md.0[1].heading, "Sub");
    assert!(
        md.0[1]
            .content
            .contains(&Paragraph::Text("More content".to_string()))
    );

    // Verify formatting includes the double newline you added in Display
    let output = md.0[0].to_string();
    assert!(output.starts_with("# Top\n\n"));
}
