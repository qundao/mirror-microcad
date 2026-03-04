// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad assignment syntax element

mod const_assignment;
mod prop_assignment;
mod value_assignment;

use crate::{src_ref::*, syntax::*, ty::*};
pub use const_assignment::*;
pub use prop_assignment::*;
pub use value_assignment::*;

/// Assignment specifying an identifier, type and value
#[derive(Clone)]
pub struct Assignment {
    /// Documentation.
    pub doc: Option<DocBlock>,
    /// Assignee
    pub(crate) id: Identifier,
    /// Type of the assignee
    pub specified_type: Option<TypeAnnotation>,
    /// Value to assign
    pub expression: Expression,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Assignment {
    /// Create new assignment.
    pub fn new(
        doc: Option<DocBlock>,
        id: Identifier,
        specified_type: Option<TypeAnnotation>,
        expression: Expression,
        src_ref: SrcRef,
    ) -> Self {
        Self {
            doc,
            id,
            specified_type,
            expression,
            src_ref,
        }
    }
}

impl Identifiable for Assignment {
    fn id_ref(&self) -> &Identifier {
        &self.id
    }
}

impl SrcReferrer for Assignment {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(
                f,
                "{id}: {ty} = {expr}",
                id = self.id,
                ty = t.ty(),
                expr = self.expression
            ),
            None => write!(f, "{id} = {expr}", id = self.id, expr = self.expression),
        }
    }
}

impl std::fmt::Debug for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(
                f,
                "{id:?}: {ty:?} = {expr:?}",
                id = self.id,
                ty = t.ty(),
                expr = self.expression
            ),
            None => write!(f, "{id:?} = {expr:?}", id = self.id, expr = self.expression),
        }
    }
}

impl Doc for Assignment {
    fn doc(&self) -> Option<DocBlock> {
        self.doc.clone()
    }
}
