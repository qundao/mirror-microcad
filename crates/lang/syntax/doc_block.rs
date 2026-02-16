// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
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
pub struct DocBlock(pub Refer<Vec<String>>);

impl DocBlock {
    /// Create new doc block for builtin.
    pub fn new_builtin(comment: &str) -> Self {
        Self(Refer::none(
            comment.lines().map(|s| s.to_string()).collect(),
        ))
    }

    /// Check if this doc block is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Merge two doc blocks, e.g. for merging inner and outer docs
    pub fn merge(a: &DocBlock, b: &DocBlock) -> DocBlock {
        match (a.is_empty(), b.is_empty()) {
            (true, true) => Self::default(),
            (true, false) => b.clone(),
            (false, true) => a.clone(),
            _ => {
                let merged =
                    a.0.iter()
                        .chain(vec![String::default()].iter()) // Add an empty line
                        .chain(b.0.iter())
                        .cloned()
                        .collect::<Vec<_>>();
                Self(Refer::new(
                    merged,
                    SrcRef::merge(&a.src_ref(), &b.src_ref()),
                ))
            }
        }
    }

    /// Remove `///` comment marks and return them as string.
    pub fn fetch_text(&self) -> String {
        self.0
            .iter()
            .filter_map(|s| s.strip_prefix("/// ").or(s.strip_prefix("///")))
            .map(|s| s.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl SrcReferrer for DocBlock {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.value.join("\n"))
    }
}

impl TreeDisplay for DocBlock {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}DocBlock: '{}'",
            "",
            crate::shorten!(self.0.first().cloned().unwrap_or_default())
        )
    }
}
