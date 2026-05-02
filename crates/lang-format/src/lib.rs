// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, virtual_url};
use microcad_syntax::{Source, ast};

mod expression;
mod extras;
mod literal;
mod node;
mod statement;
mod ty;

pub(crate) use crate::node::{BreakMode, Node};

#[derive(Debug, Clone)]
pub struct FormatConfig {
    pub max_width: usize,
    pub indent_width: usize,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            max_width: 60,
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
                node!(Node::Softline "/* " Node::from(line.clone()) " */" Node::Softline)
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

impl Format for ast::Program {
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
pub fn format(program: &ast::Program, config: &FormatConfig) -> String {
    program.format(config).to_string()
}

/// High-level API to format a &str containing µcad source code.
pub fn format_str(source: &str, config: &FormatConfig) -> Result<String, Diagnostics> {
    let source = Source::new(virtual_url(), source.to_string())?;
    Ok(format(&source.ast, config))
}

/// Format a [Source]
pub fn format_source(source: Source, config: &FormatConfig) -> Result<Source, Diagnostics> {
    Source::new(source.url.clone(), format(&source.ast, config))
}
