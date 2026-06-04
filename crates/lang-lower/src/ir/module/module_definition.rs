// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use std::rc::Rc;

use microcad_lang_base::{Identifier, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

use crate::ir;

/// Module definition.
#[derive(Clone, Debug, Default, Identifiable)]
pub struct ModuleDefinition {
    /// SrcRef of the `mod` keyword
    pub keyword_ref: SrcRef,
    /// Outer documentation.
    pub doc: ir::DocBlock,
    /// Visibility of the module.
    pub visibility: ir::Visibility,
    /// Name of the module.
    pub(crate) id: ir::Identifier,
    /// Module body. ('None' if file module)
    pub body: Option<ir::Body>,
}

impl ModuleDefinition {
    /// Create a new module definition.
    pub fn new(visibility: ir::Visibility, id: Identifier) -> Rc<Self> {
        Rc::new(Self {
            visibility,
            id,
            ..Default::default()
        })
    }
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl std::fmt::Display for ModuleDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{visibility}mod {id}",
            id = self.id,
            visibility = self.visibility,
        )
    }
}
