// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A markdown code block.

use crate::parser::ParseError;

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
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ok" => Ok(Self::Ok),
            "fail" => Ok(Self::Fail),
            "warn" => Ok(Self::Warn),
            "todo" => Ok(Self::Todo),
            "todo_fail" => Ok(Self::TodoFail),
            "todo_warn" => Ok(Self::TodoWarn),
            s => Err(ParseError::InvalidTestResult(s.to_string())),
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
    pub name: Option<String>,
    /// An optional test result
    pub test_result: Option<TestResult>,
    /// Parameters of the code block inside `()`
    pub parameters: Vec<String>,
}

/// A code block header with, e.g.: `µcad#ok(hires)`
impl CodeBlockHeader {
    /// Test banner to show test result and access logs.
    pub fn test_banner_string(name: &str) -> String {
        format!("[![test](.test/{name}.svg)](.test/{name}.log)")
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
}

impl std::fmt::Display for CodeBlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => writeln!(f, "{}\n", Self::test_banner_string(name))?,
            None => {}
        }

        write!(f, "```µcad")?;
        match &self.name {
            Some(name) => write!(f, ",{name}")?,
            None => {}
        };

        match &self.test_result {
            None | Some(TestResult::Ok) => {}
            Some(test_result) => {
                write!(f, "#{test_result}")?;
            }
        };
        if !self.parameters.is_empty() {
            write!(f, "({})", self.parameters.join(","))?;
        }
        Ok(())
    }
}

/// A code block starting inside ```
#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    /// The header of code block starting with ```
    pub header: CodeBlockHeader,
    /// The actual code.
    pub code: String,
    /// Line offset inside markdown file.
    pub line_offset: usize,
}

impl CodeBlock {
    /// Return the name of this code block.
    ///
    /// Must be unique within a markdown file.
    pub fn name(&self) -> &Option<String> {
        &self.header.name
    }

    /// Return test result.
    pub fn test_result(&self) -> &Option<TestResult> {
        &self.header.test_result
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn line_offset(&self) -> usize {
        self.line_offset
    }

    /// Returns true if this code block can be formatted.
    ///
    /// A code block can be formatted if there is no `no_format` parameter given.
    pub fn can_format(&self) -> bool {
        !self.header.parameters.contains(&String::from("no_format"))
    }
}

impl std::fmt::Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.header)?;
        if !self.code.is_empty() {
            writeln!(f, "{}", self.code)?;
        }
        write!(f, "```")
    }
}
