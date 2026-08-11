// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use crate::ir;

use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Hash, Identifiable, PartialEq, Serialize, Deserialize)]
pub struct FileModule<Path: ir::PathSpec = ir::Path> {
    pub src_ref: SrcRef,
    pub attr: ir::OuterAttributes<Path>,

    pub visibility: ir::Visibility,

    pub keyword_src_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,
}

/// Items inside an inline module that will be resolved into Symbols.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineModuleItems<Path: ir::PathSpec = ir::Path> {
    pub modules: Box<[ir::InlineModule<Path>]>,

    pub aliases: ir::Aliases<Path>,

    pub constants: Box<[ir::Constant<Path>]>,

    pub functions: Box<[ir::Function<Path>]>,

    pub workbenches: Box<[ir::Workbench<Path>]>,
}

/// Inline module definition.
#[derive(Debug, Clone, Identifiable, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineModule<Path: ir::PathSpec = ir::Path> {
    pub src_ref: SrcRef,

    /// Outer attributes.
    pub outer_attr: ir::OuterAttributes<Path>,
    /// Visibility of the module.
    pub visibility: ir::Visibility,
    /// SrcRef of the `mod` keyword
    pub keyword_src_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,

    pub inner_attr: ir::InnerAttributes<Path>,

    pub items: ir::InlineModuleItems<Path>,
}

impl<Path: ir::PathSpec> SrcReferrer for InlineModule<Path> {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl<Path: ir::PathSpec> std::fmt::Display for InlineModule<Path> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{visibility}mod {id}",
            id = self.id,
            visibility = self.visibility,
        )
    }
}
