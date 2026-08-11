// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use crate::ir;

use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Hash, Identifiable, PartialEq, Serialize, Deserialize)]
pub struct FileModule<Name: ir::NameSpec = ir::Name> {
    pub src_ref: SrcRef,
    pub attr: ir::OuterAttributes<Name>,

    pub visibility: ir::Visibility,

    pub keyword_src_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,
}

/// Items inside an inline module that will be resolved into Symbols.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineModuleItems<Name: ir::NameSpec = ir::Name> {
    pub modules: Box<[ir::InlineModule<Name>]>,

    pub aliases: ir::Aliases<Name>,

    pub constants: Box<[ir::Constant<Name>]>,

    pub functions: Box<[ir::Function<Name>]>,

    pub workbenches: Box<[ir::Workbench<Name>]>,
}

/// Inline module definition.
#[derive(Debug, Clone, Identifiable, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineModule<Name: ir::NameSpec = ir::Name> {
    pub src_ref: SrcRef,

    /// Outer attributes.
    pub outer_attr: ir::OuterAttributes<Name>,
    /// Visibility of the module.
    pub visibility: ir::Visibility,
    /// SrcRef of the `mod` keyword
    pub keyword_src_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,

    pub inner_attr: ir::InnerAttributes<Name>,

    pub items: ir::InlineModuleItems<Name>,
}

impl<Name: ir::NameSpec> SrcReferrer for InlineModule<Name> {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl<Name: ir::NameSpec> std::fmt::Display for InlineModule<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{visibility}mod {id}",
            id = self.id,
            visibility = self.visibility,
        )
    }
}
