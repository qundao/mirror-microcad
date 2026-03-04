// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad constant assignment syntax element

use crate::{src_ref::SrcReferrer, syntax::*};
use derive_more::{Deref, DerefMut};

/// Constant assignment specifying an identifier, type and value
#[derive(Clone, Deref, DerefMut)]
pub struct ConstAssignment {
    /// Constant assignment base
    #[deref]
    #[deref_mut]
    pub(crate) assignment: Assignment,
    /// Value's visibility
    pub(crate) visibility: Visibility,
}

impl ConstAssignment {
    /// Create new constant assignment.
    pub fn new(visibility: Visibility, assignment: Assignment) -> Self {
        Self {
            visibility,
            assignment,
        }
    }
}

impl SrcReferrer for ConstAssignment {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.assignment.src_ref()
    }
}

impl std::fmt::Display for ConstAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{vis}", vis = self.visibility)?;
        self.assignment.fmt(f)
    }
}

impl std::fmt::Debug for ConstAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{vis:?}", vis = self.visibility)?;
        self.assignment.fmt(f)
    }
}

impl TreeDisplay for ConstAssignment {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}ConstAssignment {vis}'{id}':",
            "",
            vis = self.visibility,
            id = self.id
        )?;
        depth.indent();
        if let Some(specified_type) = &self.specified_type {
            specified_type.tree_print(f, depth)?;
        }
        self.expression.tree_print(f, depth)
    }
}

impl Doc for ConstAssignment {
    fn doc(&self) -> Option<DocBlock> {
        self.doc.clone()
    }
}
