// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::ir::{self, ConstantExpression};
use microcad_lang_base::SrcRef;

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
