// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A markdown code block.

use microcad_lang_base::{Hashed, Source, Url};

/// A code block header.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlockHeader {
    /// Name of the code block.
    pub name: Option<String>,
    /// An optional fragment, e.g. to be used for test results: `#fragment`
    pub fragment: Option<String>,
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
        if let Some(name) = &self.name {
            writeln!(f, "{}\n", Self::test_banner_string(name))?
        }

        write!(f, "```{ext}", ext = microcad_lang_base::MICROCAD_EXTENSION)?;
        if let Some(name) = &self.name {
            write!(f, ",{name}")?
        }

        match &self.fragment {
            None => {}
            Some(fragment) => {
                write!(f, "#{fragment}")?;
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
    pub fn fragment(&self) -> &Option<String> {
        &self.header.fragment
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

    /// Get source for a code block
    pub fn source(&self, mut url: Url) -> Source {
        url.set_fragment(self.header.name.as_ref().map(|s| s.as_str()));
        Source {
            url,
            line_offset: self.line_offset as u32,
            code: Hashed::new(self.code.clone()),
        }
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
