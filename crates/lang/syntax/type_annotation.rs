// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type annotation.

use crate::ty::*;
use microcad_lang_base::{Refer, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

/// Type within source code.
#[derive(Clone, Debug, PartialEq, SrcReferrer)]
pub struct TypeAnnotation(pub Refer<Type>);

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl crate::ty::Ty for TypeAnnotation {
    fn ty(&self) -> Type {
        self.0.value.clone()
    }
}

impl TreeDisplay for TypeAnnotation {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}TypeAnnotation: {}", "", self.0.value)
    }
}
