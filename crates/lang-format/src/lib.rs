// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;

use microcad_syntax::ast;

mod error;
mod expression;
mod extras;
mod literal;
mod node;
mod statement;
mod ty;

pub(crate) use crate::{
    error::FormatError,
    node::{BreakMode, Node},
};

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

impl Format for ast::Identifier {
    fn format(&self, _: &FormatConfig) -> Node {
        self.name.clone().into()
    }
}

impl Format for ast::Comment {
    fn format(&self, _: &FormatConfig) -> Node {
        match &self.inner {
            ast::CommentInner::SingleLine(line) => {
                node!(Node::Softline Node::SingleLineComment(line.into()))
            }
            ast::CommentInner::MultiLine(line) => {
                node!(Node::Softline "/* " Node::from(line.clone()) " */")
            }
        }
    }
}

impl Format for ast::DocBlock {
    fn format(&self, _: &FormatConfig) -> Node {
        Node::vlist(
            self.lines.iter().cloned().map(|line| node!(line)),
            Node::Nil,
            0,
        )
    }
}

impl Format for ast::Source {
    fn format(&self, f: &FormatConfig) -> Node {
        self.statements.format(f)
    }
}

/// node! macro for syntactic suger.
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
    // Multiple formatted elements with extras: node!(f => begin, body, end)
    ($f:ident, $extras:expr => $($node:expr)*) => {
        $crate::extras::with_extras(
            &$extras,
            $f,
            $crate::Node::from(vec![
                $( $node.format($f) ),*
            ])

        )
    };
    // Multiple formatted elements: node!(f => begin, body, end)
    ($f:ident => $($node:expr)*) => {
        $crate::Node::from(vec![
            $( $node.format($f) ),*
        ])
    };
}

/// Format µcad source file.
pub fn format(source_file: &ast::Source, config: &FormatConfig) -> String {
    source_file.format(config).to_string()
}

/// High-level API to format a &str containing µcad source code.
pub fn format_str(source: &str, config: &FormatConfig) -> Result<String, FormatError> {
    let source_file = microcad_syntax::parse(source).map_err(FormatError::ParseErrors)?;
    Ok(format(&source_file.ast, config))
}

/// High-level API to format an entire mdbook.
pub fn format_mdbook(
    mdbook: &mut microcad_lang_markdown::MdBookDirectory,
    config: &FormatConfig,
) -> Result<(), FormatError> {
    let mut errors_by_file: HashMap<std::path::PathBuf, Vec<FormatError>> = HashMap::new();

    // 1. Iterate over code blocks. 'path' is the PathBuf of the .md file.
    mdbook
        .code_blocks_mut()
        .filter(|(_, code_block)| code_block.can_format())
        .for_each(|(path, code_block)| {
            if let Err(err) = format_str(&code_block.code, config) {
                errors_by_file
                    .entry(path.clone())
                    .or_default()
                    .push(FormatError::CodeBlock {
                        name: code_block.name().as_ref().cloned().unwrap_or_default(),
                        error: Box::new(err),
                    });
            } else if let Ok(formatted) = format_str(&code_block.code, config) {
                // Only update the code if formatting succeeded
                code_block.code = formatted;
            }
        });

    // 2. Persist the successfully formatted parts to disk
    mdbook.save_all()?;

    // 3. If we hit issues, return the map in the specific variant
    if !errors_by_file.is_empty() {
        return Err(FormatError::MdBook {
            src_path: mdbook.src_path.clone(),
            errors: errors_by_file,
        });
    }

    Ok(())
}
