// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::{CastInto, ir};

use derive_more::{Deref, DerefMut};
use microcad_lang_base::{IsDefault, Refer, SrcRef};

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

impl IsDefault for DocBlock {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl std::fmt::Display for DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.value.join("\n"))
    }
}

/// Metadata for a [`Model`]
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Meta<Name: ir::NameSpec = ir::Name> {
    pub name: ir::Name,
    pub expr: ir::ConstantExpression<Name>,
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<Meta<Dst>> for Meta<Src>
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
pub struct Command<Name: ir::NameSpec = ir::Name> {
    pub name: ir::Name,
    pub argument_list: ir::ArgumentList<ir::ConstantExpression<Name>>,
    pub src_ref: SrcRef,
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<Command<Dst>> for Command<Src>
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
pub struct Attributes<Name: ir::NameSpec = ir::Name> {
    /// Documentation
    pub doc: ir::DocBlock,
    /// Metadata: #[color = "red"]
    pub meta: Box<[Meta<Name>]>,
    /// Commands: #[export("file.svg")] #[deprecate(since = "0.2.0")]
    pub commands: Box<[Command<Name>]>,
    /// Tags: #[deprecated]
    pub tags: Box<[Tag]>,
}

impl<Name: ir::NameSpec> Default for Attributes<Name> {
    fn default() -> Self {
        Self {
            doc: Default::default(),
            meta: Default::default(),
            commands: Default::default(),
            tags: Default::default(),
        }
    }
}

impl<Name: ir::NameSpec> Attributes<Name> {
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

impl<Name: ir::NameSpec> IsDefault for Attributes<Name> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<Attributes<Dst>> for Attributes<Src>
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
pub struct InnerAttributes<Name: ir::NameSpec = ir::Name>(pub Attributes<Name>);

impl<Name: ir::NameSpec> InnerAttributes<Name> {
    /// Check if inner attributes are empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<InnerAttributes<Dst>> for InnerAttributes<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> InnerAttributes<Dst> {
        InnerAttributes(self.0.cast_into())
    }
}

impl<Name: ir::NameSpec> IsDefault for InnerAttributes<Name> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

/// Inner attributes (`///`, `#[...]`), usually lowered from definitions.
#[derive(Debug, Clone, Deref, DerefMut, Hash, PartialEq, Serialize, Deserialize)]
pub struct OuterAttributes<Name: ir::NameSpec = ir::Name>(pub Attributes<Name>);

impl<Name: ir::NameSpec> OuterAttributes<Name> {
    /// Check if outer attributes are empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<OuterAttributes<Dst>> for OuterAttributes<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> OuterAttributes<Dst> {
        OuterAttributes(self.0.cast_into())
    }
}

impl<Name: ir::NameSpec> IsDefault for OuterAttributes<Name> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl<Name: ir::NameSpec> From<OuterAttributes<Name>> for Attributes<Name> {
    fn from(value: OuterAttributes<Name>) -> Self {
        value.0
    }
}

impl<Name: ir::NameSpec> From<InnerAttributes<Name>> for Attributes<Name> {
    fn from(value: InnerAttributes<Name>) -> Self {
        value.0
    }
}
