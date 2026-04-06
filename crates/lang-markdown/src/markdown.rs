// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate a single markdown file for symbol.

use std::str::FromStr;

use derive_more::{Deref, DerefMut};
use thiserror::Error;

use crate::{CodeBlock, Paragraph, ParseError, Section};

#[derive(Error, Debug)]
pub enum MarkdownError {
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
}

/// Markdown struct, represented as a linear list of sections.
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Markdown(Vec<Section>);

impl Markdown {
    pub fn new(sections: Vec<Section>) -> Self {
        Self(sections)
    }
}

impl std::str::FromStr for Markdown {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        crate::parse(input)
    }
}

impl Markdown {
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, MarkdownError> {
        let input = std::fs::read_to_string(path)?;
        Markdown::from_str(&input).map_err(|err| err.into())
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
            if i > 0 && !section.is_empty() {
                writeln!(f)?;
            }
            write!(f, "{section}")?;
        }
        Ok(())
    }
}

#[test]
fn test_heading_parsing_and_display() {
    let input = "# Top\nContent\n\n## Sub\nMore content";
    let md = Markdown::from_str(input).unwrap();
    eprintln!("{md:#?}");

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
