// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::ir::{self, ConstantExpression};
use microcad_lang_base::{Refer, SrcRef};

use microcad_lang_proc_macros::SrcReferrer;

/// Block of documentation comments, starting with `/// `.
#[derive(Clone, Debug, Default, SrcReferrer)]
pub struct DocBlock(pub Refer<Vec<String>>);

impl DocBlock {
    /// Create new doc block for builtin.
    pub fn new_builtin(comment: &str) -> Self {
        Self(Refer::none(
            comment.lines().map(|s| format!("/// {s}")).collect(),
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
                use microcad_lang_base::SrcReferrer;
                let merged =
                    a.0.iter()
                        .chain([String::default()].iter()) // Add an empty line
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

    /// Remove `///` comment marks and return each line as string.
    pub fn fetch_lines(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|s| s.strip_prefix("/// ").or(s.strip_prefix("///")))
            .map(|s| s.trim_end().to_string())
            .collect::<Vec<_>>()
    }
}

impl std::fmt::Display for DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.value.join("\n"))
    }
}

/// Metadata for a [`Model`]
#[derive(Debug)]

pub struct Meta {
    pub name: ir::QualifiedName,
    pub expr: ConstantExpression,
}

#[derive(Debug)]
pub struct Command {
    pub name: ir::QualifiedName,
    pub argument_list: ir::ArgumentList<ConstantExpression>,
    pub src_ref: SrcRef,
}

#[derive(Debug)]
pub struct Tag {
    pub name: ir::Identifier,
}

#[derive(Debug, Default)]
pub struct Attributes {
    /// Documentation
    pub doc: ir::DocBlock,
    /// Metadata: #[color = "red"]
    pub meta: Box<[Meta]>,
    /// Commands: #[export("file.svg")] #[deprecate(since = "0.2.0")]
    pub commands: Box<[Command]>,
    /// Tags: #[deprecated]
    pub tags: Box<[Tag]>,
    /// Is inner attribute -> #![...] vs #[...]
    pub is_inner: bool,
}
