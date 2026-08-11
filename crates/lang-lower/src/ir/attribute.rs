// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::ir;

use derive_more::{Deref, DerefMut};
use microcad_lang_base::{Refer, SrcRef};

use microcad_lang_proc_macros::SrcReferrer;
use serde::{Deserialize, Serialize};

/// Block of documentation comments, stripped of `/// `.
#[derive(Clone, Debug, Default, Hash, SrcReferrer, PartialEq, Serialize, Deserialize)]
pub struct DocBlock(pub Refer<Box<[String]>>);

impl DocBlock {
    /// Create new doc block for builtin.
    pub fn new_builtin(comment: &str) -> Self {
        Self(Refer::none(
            comment
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .into_boxed_slice(),
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
                    merged.into_boxed_slice(),
                    SrcRef::merge(&a.src_ref(), &b.src_ref()),
                ))
            }
        }
    }
}

impl std::fmt::Display for DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.value.join("\n"))
    }
}

/// Metadata for a [`Model`]
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub name: ir::Path,
    pub expr: ir::ConstantExpression,
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub name: ir::Path,
    pub argument_list: ir::ArgumentList<ir::ConstantExpression>,
    pub src_ref: SrcRef,
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: ir::Identifier,
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    /// Documentation
    pub doc: ir::DocBlock,
    /// Metadata: #[color = "red"]
    pub meta: Box<[Meta]>,
    /// Commands: #[export("file.svg")] #[deprecate(since = "0.2.0")]
    pub commands: Box<[Command]>,
    /// Tags: #[deprecated]
    pub tags: Box<[Tag]>,
}

impl Attributes {
    pub fn is_empty(&self) -> bool {
        self.doc.is_empty()
            && self.meta.is_empty()
            && self.commands.is_empty()
            && self.tags.is_empty()
    }

    /// Consumes both `self` and `rhs`, returning a merged `Attributes` with appended fields.
    pub fn extend(mut self, rhs: Self) -> Self {
        /// Helper function to combine two `Box<[T]>` without reallocating if one side is empty.
        fn extend_boxed_slices<T>(lhs: Box<[T]>, rhs: Box<[T]>) -> Box<[T]> {
            if rhs.is_empty() {
                return lhs;
            }
            if lhs.is_empty() {
                return rhs;
            }

            let mut vec = lhs.into_vec();
            vec.extend(rhs.into_vec());
            vec.into_boxed_slice()
        }

        // Extend documentation
        self.doc = ir::DocBlock::merge(&self.doc, &rhs.doc);

        // Convert Box<[T]> to Vec<T> to append, then back to Box<[T]>
        self.meta = extend_boxed_slices(self.meta, rhs.meta);
        self.commands = extend_boxed_slices(self.commands, rhs.commands);
        self.tags = extend_boxed_slices(self.tags, rhs.tags);

        self
    }
}

/// Inner attributes (`//!`, `#![...]`), usually lowered from a `ast::StatementList`.
#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Hash, Serialize, Deserialize)]
pub struct InnerAttributes(pub Attributes);

/// Outer attributes (`///`, `#[...]`), usually lowered from definitions.
#[derive(Debug, Clone, Deref, DerefMut, Hash, PartialEq, Serialize, Deserialize)]
pub struct OuterAttributes(pub Attributes);

impl From<OuterAttributes> for Attributes {
    fn from(value: OuterAttributes) -> Self {
        value.0
    }
}

impl From<InnerAttributes> for Attributes {
    fn from(value: InnerAttributes) -> Self {
        value.0
    }
}
