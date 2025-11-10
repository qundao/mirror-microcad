// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Documentation block syntax element

use crate::{src_ref::*, syntax::*};

/// Retrieve doc from symbol definition.
pub trait Doc {
    /// Return documentation
    fn doc(&self) -> Option<DocBlock>;
}

/// Static variant of trait Doc for builtins.
pub type BuiltinDocFn = dyn Fn() -> Option<DocBlock>;

/// Block of documentation comments, starting with `/// `.
#[derive(Clone, Default)]
pub struct DocBlock {
    /// Doc summary.
    pub summary: String,
    /// Doc details.
    pub details: Option<String>,
    /// Source reference.
    pub src_ref: SrcRef,
}

impl DocBlock {
    /// Create new doc block for builtin.
    pub fn new_builtin(comment: &str) -> Self {
        let lines: Vec<_> = comment.lines().collect();
        let (summary, details) =
            if let Some(pos) = lines.iter().position(|line| line.trim().is_empty()) {
                (lines[0..pos].join("\n"), Some(lines[pos..].join("\n")))
            } else {
                (lines.join("\n"), None)
            };

        Self {
            summary,
            details,
            src_ref: SrcRef(None),
        }
    }
}

impl SrcReferrer for DocBlock {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let summary = &self.summary;
        match &self.details {
            None => write!(f, "{summary}"),
            Some(details) => write!(f, "{summary}\n\n{details}"),
        }
    }
}

impl TreeDisplay for DocBlock {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}DocBlock: '{}'",
            "",
            crate::shorten!(self.summary)
        )
    }
}
