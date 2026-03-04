// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad property assignment syntax element

use crate::{src_ref::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// Property assignment specifying an identifier, type and value
#[derive(Clone, Deref, DerefMut)]
pub struct PropAssignment(Assignment);

impl From<Assignment> for PropAssignment {
    fn from(value: Assignment) -> Self {
        Self(value)
    }
}

impl SrcReferrer for PropAssignment {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for PropAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for PropAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TreeDisplay for PropAssignment {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}PropAssignment '{id}':", "", id = self.id)?;
        depth.indent();
        if let Some(specified_type) = &self.specified_type {
            specified_type.tree_print(f, depth)?;
        }
        self.expression.tree_print(f, depth)
    }
}
