// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad markdown parser.

use crate::{
    CodeBlock, Markdown, Paragraph, Section,
    code_block::{CodeBlockHeader, TestResult},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Missing code block fence")]
    MissingCodeBlockFence,

    #[error("Unexpected end of file while parsing")]
    UnexpectedEOF,

    #[error("Malformed header")]
    MalformedHeader,

    #[error("Invalid test result: {0}")]
    InvalidTestResult(String),

    #[error("Duplicated code block name: {0}")]
    DuplicatedCodeBlockName(String),
}

pub struct ParseContext<'a> {
    current_line: Option<&'a str>,
    current_line_number: usize,
    lines: std::iter::Peekable<std::iter::Enumerate<std::str::Lines<'a>>>,
}

impl<'a> ParseContext<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            current_line: None,
            current_line_number: 0,
            lines: input.lines().enumerate().peekable(),
        }
    }

    pub(crate) fn next(&mut self) -> Option<(usize, &'a str)> {
        let next = self.lines.next();
        match &next {
            Some((line_number, line)) => {
                self.current_line_number = *line_number;
                self.current_line = Some(line);
            }
            None => {
                self.current_line_number = 0;
                self.current_line = None;
            }
        }
        next
    }
}

pub trait Parse
where
    Self: Sized,
{
    fn parse(context: &mut ParseContext) -> Result<Self, ParseError>;
}

impl Parse for CodeBlockHeader {
    fn parse(context: &mut ParseContext) -> Result<Self, ParseError> {
        // 1. Consume optional test banner and any subsequent empty lines
        let line = context.current_line.as_ref().expect("A current line");

        if Self::is_test_banner(line) {
            context.lines.next();
            while let Some((_, next_line)) = context.lines.peek() {
                if next_line.trim().is_empty() {
                    context.lines.next();
                } else {
                    break;
                }
            }
        }

        // 2. Consume and validate the fence line (e.g., ```µcad,name#ok(param))
        let (_, header_line) = context.lines.peek().ok_or(ParseError::UnexpectedEOF)?;
        let trimmed = header_line.trim();
        assert!(trimmed.starts_with("```"));

        // Metadata is everything after "```"
        let meta = &trimmed[3..];

        // 3. Locate structural delimiters
        let hash_pos = meta.find('#');
        let paren_pos = meta.find('(');

        // 4. Parse Name (supports "µcad,my_name" or just "my_name")
        let name_end = hash_pos.or(paren_pos).unwrap_or(meta.len());
        let name_part = meta[..name_end].trim();
        let name = if let Some(comma_idx) = name_part.find(',') {
            name_part[comma_idx + 1..].trim().to_string()
        } else {
            name_part.to_string()
        };

        // 5. Parse TestResult (#ok, #fail, etc.)
        let mut test_result = Some(TestResult::Ok);
        if let Some(start) = hash_pos {
            let end = paren_pos.unwrap_or(meta.len());
            let status_str = meta[start + 1..end].trim();
            use std::str::FromStr;
            test_result = Some(TestResult::from_str(status_str)?);
        }

        // 6. Parse Parameters ((hires, lowres))
        let mut parameters = Vec::new();
        if let Some(start) = paren_pos {
            let end = meta.find(')').ok_or_else(|| ParseError::MalformedHeader)?;

            parameters = meta[start + 1..end]
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        Ok(Self {
            name,
            test_result,
            parameters,
        })
    }
}

impl Parse for CodeBlock {
    fn parse(context: &mut ParseContext) -> Result<Self, ParseError> {
        let mut code_lines = Vec::new();
        let mut closed = false;

        let header = CodeBlockHeader::parse(context)?;
        let mut start_line_no = None;
        context.next();

        // Consume until closing backticks
        while let Some((idx, line)) = context.next() {
            if start_line_no.is_none() {
                start_line_no = Some(idx);
            }

            if line.trim().starts_with("```") {
                closed = true;
                break;
            }
            code_lines.push(line);
        }

        if !closed {
            return Err(ParseError::UnexpectedEOF);
        }

        Ok(Self {
            header,
            code: code_lines.join("\n"),
            line_offset: start_line_no.expect("Some line"),
        })
    }
}

impl Parse for Markdown {
    fn parse(context: &mut ParseContext) -> Result<Self, ParseError> {
        let mut sections = Vec::new();
        let mut current_section = Section::default();

        let mut code_block_names = std::collections::HashSet::new();

        while let Some((_, line)) = context.next() {
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
                let block = CodeBlock::parse(context)?;
                let block_name = block.name().to_string();
                if code_block_names.contains(block.name()) {
                    return Err(ParseError::DuplicatedCodeBlockName(block_name));
                } else {
                    code_block_names.insert(block_name);
                }

                current_section.content.push(Paragraph::CodeBlock(block));
            }
            // 3. Tables
            else if trimmed.starts_with('|') {
                let mut content = vec![line.to_string()];
                while let Some((_, line)) = context.lines.next() {
                    let trimmed = line.trim();
                    if !trimmed.starts_with("|") {
                        break;
                    }
                    content.push(line.to_string());
                }
                current_section
                    .content
                    .push(Paragraph::Table(content.join("\n").trim().to_string()));
            }
            // 4. Text
            else {
                let mut content = vec![line.to_string()];
                while let Some((_, line)) = context.next() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        break;
                    }
                    content.push(line.to_string());
                }
                current_section
                    .content
                    .push(Paragraph::Text(content.join("\n").trim().to_string()));
            }
        }

        sections.push(current_section);
        Ok(Self::new(sections))
    }
}

/// Parse a markdown.
pub fn parse(input: &str) -> Result<Markdown, ParseError> {
    let mut context = ParseContext::new(input);
    Markdown::parse(&mut context)
}
