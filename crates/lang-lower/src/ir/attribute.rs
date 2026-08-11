// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::{CastInto, ir};

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
pub struct Meta<Path: ir::PathSpec = ir::Path> {
    pub name: ir::Path,
    pub expr: ir::ConstantExpression<Path>,
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<Meta<Dst>> for Meta<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> Meta<Dst> {
        Meta {
            name: self.name,
            expr: self.expr.cast_into(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Command<Path: ir::PathSpec = ir::Path> {
    pub name: ir::Path,
    pub argument_list: ir::ArgumentList<ir::ConstantExpression<Path>>,
    pub src_ref: SrcRef,
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<Command<Dst>> for Command<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> Command<Dst> {
        Command {
            name: self.name,
            argument_list: self.argument_list.cast_into(),
            src_ref: self.src_ref,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: ir::Identifier,
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Attributes<Path: ir::PathSpec = ir::Path> {
    /// Documentation
    pub doc: ir::DocBlock,
    /// Metadata: #[color = "red"]
    pub meta: Box<[Meta<Path>]>,
    /// Commands: #[export("file.svg")] #[deprecate(since = "0.2.0")]
    pub commands: Box<[Command<Path>]>,
    /// Tags: #[deprecated]
    pub tags: Box<[Tag]>,
}

impl<Path: ir::PathSpec> Default for Attributes<Path> {
    fn default() -> Self {
        Self {
            doc: Default::default(),
            meta: Default::default(),
            commands: Default::default(),
            tags: Default::default(),
        }
    }
}

impl<Path: ir::PathSpec> Attributes<Path> {
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

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<Attributes<Dst>> for Attributes<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> Attributes<Dst> {
        Attributes {
            doc: self.doc,
            meta: self.meta.cast_into(),
            commands: self.commands.cast_into(),
            tags: self.tags,
        }
    }
}

/// Inner attributes (`//!`, `#![...]`), usually lowered from a `ast::StatementList`.
#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Hash, Serialize, Deserialize)]
pub struct InnerAttributes<Path: ir::PathSpec = ir::Path>(pub Attributes<Path>);

impl<Path: ir::PathSpec> InnerAttributes<Path> {
    /// Check if inner attributes are empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<InnerAttributes<Dst>> for InnerAttributes<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> InnerAttributes<Dst> {
        InnerAttributes(self.0.cast_into())
    }
}

/// Outer attributes (`///`, `#[...]`), usually lowered from definitions.
#[derive(Debug, Clone, Deref, DerefMut, Hash, PartialEq, Serialize, Deserialize)]
pub struct OuterAttributes<Path: ir::PathSpec = ir::Path>(pub Attributes<Path>);

impl<Path: ir::PathSpec> OuterAttributes<Path> {
    /// Check if outer attributes are empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<OuterAttributes<Dst>> for OuterAttributes<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> OuterAttributes<Dst> {
        OuterAttributes(self.0.cast_into())
    }
}

impl<Path: ir::PathSpec> From<OuterAttributes<Path>> for Attributes<Path> {
    fn from(value: OuterAttributes<Path>) -> Self {
        value.0
    }
}

impl<Path: ir::PathSpec> From<InnerAttributes<Path>> for Attributes<Path> {
    fn from(value: InnerAttributes<Path>) -> Self {
        value.0
    }
}
