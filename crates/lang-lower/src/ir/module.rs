// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;
use serde::Serialize;

use crate::IsDefault;
use crate::ir;
use crate::is_default;

#[derive(Debug, Identifiable, Serialize)]
pub struct FileModule {
    pub src_ref: SrcRef,
    pub attr: ir::OuterAttributes,

    pub visibility: ir::Visibility,

    pub keyword_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,
}

#[derive(Debug, Default, Serialize)]
pub struct FileModules(pub Box<[FileModule]>);

impl IsDefault for FileModules {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}

/// Items inside an inline module that will be resolved into Symbols.
#[derive(Debug, Serialize)]
pub struct InlineModuleItems {
    #[serde(skip_serializing_if = "is_default", default)]
    pub modules: ir::InlineModules,

    #[serde(skip_serializing_if = "is_default", default)]
    pub aliases: ir::Aliases,

    #[serde(skip_serializing_if = "is_default", default)]
    pub constants: ir::Constants,

    #[serde(skip_serializing_if = "is_default", default)]
    pub functions: ir::Functions,

    #[serde(skip_serializing_if = "is_default", default)]
    pub workbenches: ir::Workbenches,
}

impl IsDefault for InlineModuleItems {
    fn is_default(&self) -> bool {
        self.modules.is_default()
            && self.aliases.is_default()
            && self.constants.is_default()
            && self.functions.is_default()
            && self.workbenches.is_default()
    }
}

/// Inline module definition.
#[derive(Debug, Identifiable, Serialize)]
pub struct InlineModule {
    pub src_ref: SrcRef,

    /// Outer attributes.
    #[serde(skip_serializing_if = "is_default", default)]
    pub outer_attr: ir::OuterAttributes,
    /// Visibility of the module.
    pub visibility: ir::Visibility,
    /// SrcRef of the `mod` keyword
    pub keyword_ref: SrcRef,
    /// Name of the module.
    pub id: ir::Identifier,

    #[serde(skip_serializing_if = "is_default", default)]
    pub inner_attr: ir::InnerAttributes,

    #[serde(skip_serializing_if = "is_default", default)]
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

#[derive(Debug, Default, Serialize)]
pub struct InlineModules(pub Box<[InlineModule]>);

impl IsDefault for InlineModules {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}
