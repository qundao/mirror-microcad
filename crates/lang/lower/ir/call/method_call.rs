// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Method call syntax elements.

use crate::lower::ir;

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

/// Method call syntax entity.
#[derive(Clone, Debug, SrcReferrer)]
pub struct MethodCall {
    /// Name of the method.
    pub name: ir::QualifiedName,
    /// List of arguments.
    pub argument_list: ir::ArgumentList,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}
