// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::{src_ref::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// *Command syntax* within an attribute.
#[derive(Clone)]
pub enum AttributeCommand {
    /// A bare name
    Ident(Identifier),
    /// A command with optional arguments: `width(offset = 30mm)`.
    Call(Call),
    /// A assigment: `color = "red"`.
    Assigment {
        /// Source code reference.
        src_ref: SrcRef,
        /// Name of the assigment.
        name: Identifier,
        /// Value name of the assigment.
        value: Expression,
    },
}

impl AttributeCommand {
    /// Qualified name of the attribute command
    pub fn name(&self) -> &Identifier {
        match self {
            AttributeCommand::Ident(name) => name,
            AttributeCommand::Call(call) => &call
                .name
                .as_identifier()
                .expect("non-identifier attribute call"),
            AttributeCommand::Assigment { name, .. } => name,
        }
    }
}

impl std::fmt::Display for AttributeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AttributeCommand::Ident(name) => write!(f, "{name}"),
            AttributeCommand::Call(call) => write!(f, "{call}"),
            AttributeCommand::Assigment { name, value, .. } => write!(f, "{name} = {value}"),
        }
    }
}

impl std::fmt::Debug for AttributeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AttributeCommand::Ident(name) => write!(f, "{name:?}"),
            AttributeCommand::Call(call) => write!(f, "{call:?}"),
            AttributeCommand::Assigment { name, value, .. } => write!(f, "{name:?} = {value:?}"),
        }
    }
}

impl SrcReferrer for AttributeCommand {
    fn src_ref(&self) -> SrcRef {
        match &self {
            AttributeCommand::Ident(name) => name.src_ref(),
            AttributeCommand::Call(call) => call.src_ref(),
            AttributeCommand::Assigment { src_ref, .. } => src_ref.clone(),
        }
    }
}

/// An attribute item.
#[derive(Clone)]
pub struct Attribute {
    /// Attribute commands: `export = "test.stl", height(30mm)`.
    pub commands: Vec<AttributeCommand>,
    /// Tells if the attribute is an inner attribute: `#[...]` (outer) vs `#![...]` (inner).
    pub is_inner: bool,
    /// Source reference
    pub src_ref: SrcRef,
}

impl Attribute {
    /// Return some command it is the only one in the list.
    pub fn single_command(&self) -> Option<&AttributeCommand> {
        match self.commands.len() {
            1 => self.commands.first(),
            _ => None,
        }
    }
}

impl TreeDisplay for Attribute {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Attribute: {self}", "")
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.is_inner {
            true => write!(f, "#![")?,
            false => write!(f, "#[")?,
        }
        write!(
            f,
            "{}",
            self.commands
                .iter()
                .map(|command| command.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        writeln!(f, "]")
    }
}

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl SrcReferrer for Attribute {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

/// A list of attributes, e.g. `#foo #[bar, baz = 42]`
#[derive(Clone, Default, Deref, DerefMut)]
pub struct AttributeList(Vec<Attribute>);

impl std::fmt::Display for AttributeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|attr| writeln!(f, "{attr}"))
    }
}

impl std::fmt::Debug for AttributeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl SrcReferrer for AttributeList {
    fn src_ref(&self) -> SrcRef {
        if self.0.is_empty() {
            SrcRef(None)
        } else {
            SrcRef::merge(
                &self.0.first().expect("One element").src_ref(),
                &self.0.last().expect("Second element").src_ref(),
            )
        }
    }
}
