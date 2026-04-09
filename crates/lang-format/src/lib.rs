// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;

use microcad_syntax::ast;

mod error;
mod expression;
mod literal;
mod node;
mod statement;
mod ty;

use crate::{error::FormatError, node::Node};

#[derive(Debug, Clone)]
pub struct FormatConfig {
    pub max_width: usize,
    pub indent_width: usize,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            max_width: 80,
            indent_width: 4,
        }
    }
}

pub(crate) trait Format {
    fn format(&self, f: &FormatConfig) -> Node;
}

impl Format for ast::Identifier {
    fn format(&self, _: &FormatConfig) -> Node {
        self.name.clone().into()
    }
}

impl Format for ast::Comment {
    fn format(&self, _: &FormatConfig) -> Node {
        match &self.inner {
            ast::CommentInner::SingleLine(items) => Node::vlist(
                items.into_iter().cloned().map(|item| item.into()),
                Node::Nil,
            ),
            ast::CommentInner::MultiLine(line) => node!("/*" Node::from(line.clone()) "*/"),
        }
    }
}

#[macro_export]
macro_rules! node {
    // Single element: node!(x)
    ($node:expr) => {
        $crate::Node::from($node)
    };
    // Multiple elements: node!(begin, body, end)
    ($($node:expr)*) => {
        $crate::Node::from(vec![
            $( $crate::Node::from($node) ),*
        ])
    };
    // Multiple formatted elements: node!(f => begin, body, end)
    ($f:ident => $($node:expr)*) => {
        $crate::Node::from(vec![
            $( $node.format($f) ),*
        ])
    };

}

impl<T> Format for T
where
    T: Into<Node> + Clone,
{
    fn format(&self, _f: &FormatConfig) -> Node {
        self.clone().into()
    }
}

// Blanket impl for Option
impl<T: Format> Format for Option<T> {
    fn format(&self, f: &FormatConfig) -> Node {
        match self {
            Some(inner) => inner.format(f),
            None => Node::Nil, // Or whatever your "null" node is
        }
    }
}

impl Format for ast::SourceFile {
    fn format(&self, f: &FormatConfig) -> Node {
        self.statements.format(f)
    }
}

/// Format µcad source file.
pub fn format(source_file: &ast::SourceFile, config: &FormatConfig) -> String {
    source_file.format(config).to_string()
}

/// High-level API to format a &str containing µcad source code.
pub fn format_str(source: &str, config: &FormatConfig) -> Result<String, FormatError> {
    let tokens: Vec<_> = microcad_syntax::lex(&source).collect();
    let source_file =
        microcad_syntax::parse(&tokens).map_err(|err| FormatError::ParseErrors(err))?;
    Ok(format(&source_file, &config))
}

/// High-level API to format an entire mdbook.
///
/// TODO: needs proper error handling.
pub fn format_mdbook(
    mdbook: &mut microcad_lang_markdown::MdBookDirectory,
    config: &FormatConfig,
) -> Result<(), FormatError> {
    let mut errors_by_file: HashMap<std::path::PathBuf, Vec<FormatError>> = HashMap::new();

    // 1. Iterate over code blocks. 'path' is the PathBuf of the .md file.
    mdbook.code_blocks_mut().for_each(|(path, code_block)| {
        if let Err(err) = format_str(&code_block.code, config) {
            errors_by_file.entry(path.clone()).or_default().push(
                FormatError::CodeBlockFormatError {
                    name: code_block.name().as_ref().cloned().unwrap_or_default(),
                    error: Box::new(err),
                },
            );
        } else if let Ok(formatted) = format_str(&code_block.code, config) {
            // Only update the code if formatting succeeded
            code_block.code = formatted;
        }
    });

    // 2. Persist the successfully formatted parts to disk
    mdbook.save_all()?;

    // 3. If we hit issues, return the map in the specific variant
    if !errors_by_file.is_empty() {
        return Err(FormatError::MdBookFormatError {
            src_path: mdbook.src_path.clone(),
            errors: errors_by_file,
        });
    }

    Ok(())
}
