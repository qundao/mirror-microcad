// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Assignment statement syntax elements

use crate::{rc::*, src_ref::*, syntax::*};

/// An assignment statement, e.g. `#[aux] s = Sphere(3.0mm);`.
#[derive(Clone)]
pub struct AssignmentStatement<T> {
    /// List of attributes.
    pub attribute_list: AttributeList,
    /// The actual assignment.
    pub assignment: Rc<T>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<T> SrcReferrer for AssignmentStatement<T>
where
    T: std::fmt::Display,
{
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl<T> TreeDisplay for AssignmentStatement<T>
where
    T: std::fmt::Display,
{
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Assignment {}", "", self.assignment)
    }
}

impl<T> std::fmt::Display for AssignmentStatement<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            write!(f, "{} ", self.attribute_list)?;
        }
        write!(f, "{};", self.assignment)
    }
}

impl<T> std::fmt::Debug for AssignmentStatement<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            write!(f, "{:?} ", self.attribute_list)?;
        }
        write!(f, "{:?};", self.assignment)
    }
}
