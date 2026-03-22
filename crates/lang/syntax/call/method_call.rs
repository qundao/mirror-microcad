// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Method call syntax elements.

use microcad_lang_base::{SrcRef, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

use crate::syntax::*;

/// Method call syntax entity.
#[derive(Clone, Debug, SrcReferrer)]
pub struct MethodCall {
    /// Name of the method.
    pub name: QualifiedName,
    /// List of arguments.
    pub argument_list: ArgumentList,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}

impl TreeDisplay for MethodCall {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}MethodCall '{}':", "", self.name)?;
        depth.indent();
        self.argument_list.tree_print(f, depth)
    }
}
