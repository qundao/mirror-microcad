// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

use crate::ir;

#[derive(Debug, Identifiable)]
pub struct FileModule {
    pub src_ref: SrcRef,
    pub attr: ir::Attributes,

    pub visibility: ir::Visibility,

    pub keyword_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,
}

#[derive(Debug, Default)]
pub struct FileModules(pub Box<[FileModule]>);

/// Module definition.
#[derive(Debug, Identifiable)]
pub struct InlineModule {
    pub src_ref: SrcRef,

    /// Outer documentation.
    pub outer_attr: ir::Attributes,
    /// Visibility of the module.
    pub visibility: ir::Visibility,
    /// SrcRef of the `mod` keyword
    pub keyword_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,

    pub inner_attr: ir::Attributes,

    pub modules: ir::InlineModules,

    pub aliases: ir::Aliases,

    pub constants: ir::Constants,

    pub functions: ir::Functions,

    pub workbenches: ir::Workbenches,
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

#[derive(Debug, Default)]
pub struct InlineModules(pub Box<[InlineModule]>);
