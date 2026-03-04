// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad value assignment syntax element

use crate::{src_ref::SrcReferrer, syntax::*};
use derive_more::{Deref, DerefMut};

/// Value assignment specifying an identifier, type and value
#[derive(Clone, Deref, DerefMut)]
pub struct ValueAssignment(Assignment);

impl From<Assignment> for ValueAssignment {
    fn from(value: Assignment) -> Self {
        Self(value)
    }
}

impl SrcReferrer for ValueAssignment {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for ValueAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for ValueAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TreeDisplay for Assignment {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Value Assignment '{id}':", "", id = self.id)?;
        depth.indent();
        if let Some(specified_type) = &self.specified_type {
            specified_type.tree_print(f, depth)?;
        }
        self.expression.tree_print(f, depth)
    }
}
