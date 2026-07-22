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
pub struct Meta<NAME: Serialize = ir::SymbolPath> {
    pub name: ir::SymbolPath,
    pub expr: ir::ConstantExpression<NAME>,
}

impl<T: Serialize, NAME: Serialize> CastInto<Meta<T>> for Meta<NAME>
where
    NAME: Into<T>,
{
    fn cast_into(self) -> Meta<T> {
        Meta {
            name: self.name,
            expr: self.expr.cast_into(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Command<NAME: Serialize = ir::SymbolPath> {
    pub name: ir::SymbolPath,
    pub argument_list: ir::ArgumentList<ir::ConstantExpression<NAME>>,
    pub src_ref: SrcRef,
}

impl<T: Serialize, NAME: Serialize> CastInto<Command<T>> for Command<NAME>
where
    NAME: Into<T>,
{
    fn cast_into(self) -> Command<T> {
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
pub struct Attributes<NAME: Serialize = ir::SymbolPath> {
    /// Documentation
    pub doc: ir::DocBlock,
    /// Metadata: #[color = "red"]
    pub meta: Box<[Meta<NAME>]>,
    /// Commands: #[export("file.svg")] #[deprecate(since = "0.2.0")]
    pub commands: Box<[Command<NAME>]>,
    /// Tags: #[deprecated]
    pub tags: Box<[Tag]>,
}

impl<NAME: Serialize> Attributes<NAME> {
    pub fn is_empty(&self) -> bool {
        self.doc.is_empty()
            && self.meta.is_empty()
            && self.commands.is_empty()
            && self.tags.is_empty()
    }
}

impl<NAME: Serialize> IsDefault for Attributes<NAME> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl<T: Serialize, NAME: Serialize> CastInto<Attributes<T>> for Attributes<NAME>
where
    NAME: Into<T>,
{
    fn cast_into(self) -> Attributes<T> {
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
pub struct InnerAttributes<NAME: Serialize = ir::SymbolPath>(pub Attributes<NAME>);

impl<NAME: Serialize> InnerAttributes<NAME> {
    /// Check if inner attributes are empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Serialize, NAME: Serialize> CastInto<InnerAttributes<T>> for InnerAttributes<NAME>
where
    NAME: Into<T>,
{
    fn cast_into(self) -> InnerAttributes<T> {
        InnerAttributes(self.0.cast_into())
    }
}

impl<NAME: Serialize> IsDefault for InnerAttributes<NAME> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

/// Inner attributes (`///`, `#[...]`), usually lowered from definitions.
#[derive(Debug, Clone, Deref, DerefMut, Hash, PartialEq, Serialize, Deserialize)]
pub struct OuterAttributes<NAME: Serialize = ir::SymbolPath>(pub Attributes<NAME>);

impl<NAME: Serialize> OuterAttributes<NAME> {
    /// Check if outer attributes are empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Serialize, NAME: Serialize> CastInto<OuterAttributes<T>> for OuterAttributes<NAME>
where
    NAME: Into<T>,
{
    fn cast_into(self) -> OuterAttributes<T> {
        OuterAttributes(self.0.cast_into())
    }
}

impl<NAME: Serialize> IsDefault for OuterAttributes<NAME> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl<NAME: Serialize> From<OuterAttributes<NAME>> for Attributes<NAME> {
    fn from(value: OuterAttributes<NAME>) -> Self {
        value.0
    }
}

impl<NAME: Serialize> From<InnerAttributes<NAME>> for Attributes<NAME> {
    fn from(value: InnerAttributes<NAME>) -> Self {
        value.0
    }
}
