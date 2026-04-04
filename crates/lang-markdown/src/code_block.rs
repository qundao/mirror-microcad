// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A markdown code block.

use std::str::FromStr;

use crate::markdown::MarkdownError;

/// Markdown test result: `ok, fail, warn, todo` etc.
#[derive(Debug, Clone, PartialEq)]
pub enum TestResult {
    /// Ok
    Ok,
    /// Expected to fail
    Fail,
    /// Warnings expected
    Warn,
    /// Work in progress
    Todo,
    /// Work in progress (should fail)
    TodoFail,
    /// Work in progress (should have warnings)
    TodoWarn,
}

impl std::str::FromStr for TestResult {
    type Err = MarkdownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ok" => Ok(Self::Ok),
            "fail" => Ok(Self::Fail),
            "warn" => Ok(Self::Warn),
            "todo" => Ok(Self::Todo),
            "todo_fail" => Ok(Self::TodoFail),
            "todo_warn" => Ok(Self::TodoWarn),
            s => Err(MarkdownError::InvalidTestResult(s.to_string())),
        }
    }
}

impl std::fmt::Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                TestResult::Ok => "ok",
                TestResult::Fail => "fail",
                TestResult::Warn => "warn",
                TestResult::Todo => "todo",
                TestResult::TodoFail => "todo_fail",
                TestResult::TodoWarn => "todo_warn",
            }
        )
    }
}

/// A code block header.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlockHeader {
    /// Name of the code block.
    name: String,
    /// An optional test result
    test_result: Option<TestResult>,
    /// Parameters if the code block inside `()`
    parameters: Vec<String>,
}

impl std::fmt::Display for CodeBlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        match &self.test_result {
            Some(test_result) => {
                writeln!(f, "{}\n", Self::test_banner_string(name))?;
                write!(f, "```µcad,{name}#{test_result}")?;
            }
            None => {
                write!(f, "```µcad,{name}")?;
            }
        };

        if !self.parameters.is_empty() {
            write!(f, "({})", self.parameters.join(","))?;
        }
        Ok(())
    }
}

/// A code block header with, e.g.: `µcad#ok(hires)`
impl CodeBlockHeader {
    /// Test banner to show test result and access logs.
    fn test_banner_string(name: &str) -> String {
        format!("[![test](.test/{name}.svg)](.test/{name}.log]")
    }

    pub(crate) fn is_test_banner(line: &str) -> bool {
        line.starts_with("[![test]")
    }

    pub(crate) fn is_code_block_start(line: &str) -> bool {
        microcad_lang_base::MICROCAD_EXTENSIONS
            .iter()
            .any(|ext| line.starts_with(&format!("```{ext}")))
            || Self::is_test_banner(line)
    }

    /// Parse the header.
    ///
    /// The test banner is ignored during parsing.
    pub(crate) fn parse(
        line: &str,
        lines: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Lines<'_>>>,
    ) -> Result<Self, MarkdownError> {
        // 1. Consume optional test banner and any subsequent empty lines
        if Self::is_test_banner(line) {
            lines.next();
            while let Some((_, next_line)) = lines.peek() {
                if next_line.trim().is_empty() {
                    lines.next();
                } else {
                    break;
                }
            }
        }

        // 2. Consume and validate the fence line (e.g., ```µcad,name#ok(param))
        let (_, header_line) = lines.peek().ok_or(MarkdownError::UnexpectedEOF)?;
        let trimmed = header_line.trim();

        if !trimmed.starts_with("```") {
            return Err(MarkdownError::MissingFence);
        }

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
        let mut test_result = None;
        if let Some(start) = hash_pos {
            let end = paren_pos.unwrap_or(meta.len());
            let status_str = meta[start + 1..end].trim();

            test_result = Some(TestResult::from_str(status_str)?);
        }

        // 6. Parse Parameters ((hires, lowres))
        let mut parameters = Vec::new();
        if let Some(start) = paren_pos {
            let end = meta
                .find(')')
                .ok_or_else(|| MarkdownError::MalformedHeader)?;

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

/// A code block starting inside ```
#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    /// The header of code block starting with ```
    header: CodeBlockHeader,
    /// The actual code.
    code: String,
    /// Start line
    start_line_no: usize,
}

impl CodeBlock {
    /// Return the name of this code block.
    ///
    /// Must be unique within a markdown file.
    pub fn name(&self) -> &String {
        &self.header.name
    }

    pub(crate) fn parse(
        line: &str,
        lines: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Lines<'_>>>,
    ) -> Result<Self, MarkdownError> {
        let mut code_lines = Vec::new();
        let mut closed = false;

        let header = CodeBlockHeader::parse(line, lines)?;
        let mut start_line_no = None;

        // Consume until closing backticks
        while let Some((idx, line)) = lines.next() {
            if start_line_no.is_none() {
                start_line_no = Some(idx)
            }

            if line.trim().starts_with("```") {
                closed = true;
                break;
            }
            code_lines.push(line);
        }

        if !closed {
            return Err(MarkdownError::UnexpectedEOF);
        }

        Ok(Self {
            header,
            code: code_lines.join("\n"),
            start_line_no: start_line_no.expect("Some line"),
        })
    }
}

impl std::fmt::Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{}\n```", self.code)
    }
}
