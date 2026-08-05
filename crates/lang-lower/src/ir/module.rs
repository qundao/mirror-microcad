// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use crate::ir;

use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Hash, Identifiable, PartialEq, Serialize, Deserialize)]
pub struct FileModule {
    pub src_ref: SrcRef,
    pub attr: ir::OuterAttributes,

    pub visibility: ir::Visibility,

    pub keyword_src_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,
}

/// Items inside an inline module that will be resolved into Symbols.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineModuleItems {
    pub modules: Box<[ir::InlineModule]>,

    pub aliases: ir::Aliases,

    pub constants: Box<[ir::Constant]>,

    pub functions: Box<[ir::Function]>,

    pub workbenches: Box<[ir::Workbench]>,
}

/// Inline module definition.
#[derive(Debug, Clone, Identifiable, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineModule {
    pub src_ref: SrcRef,

    /// Outer attributes.
    pub outer_attr: ir::OuterAttributes,
    /// Visibility of the module.
    pub visibility: ir::Visibility,
    /// SrcRef of the `mod` keyword
    pub keyword_src_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,

    pub inner_attr: ir::InnerAttributes,

    pub items: ir::InlineModuleItems,
}

impl SrcReferrer for InlineModule {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl std::fmt::Display for InlineModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{visibility}mod {id}",
            id = self.id,
            visibility = self.visibility,
        )
    }
}
