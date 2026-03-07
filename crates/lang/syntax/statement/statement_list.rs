// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Statement list syntax element.

use crate::{src_ref::*, syntax::*};

/// A list of statements.
#[derive(Clone, Default)]
pub struct StatementList {
    pub statements: Vec<Statement>,
    pub tail: Option<Box<Statement>>,
}

impl StatementList {
    pub fn iter(&self) -> impl Iterator<Item = &Statement> {
        self.statements.iter().chain(self.tail.as_deref())
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }
}

impl std::fmt::Display for StatementList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.iter() {
            writeln!(f, "{statement}")?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for StatementList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.iter() {
            writeln!(f, "{statement:?}")?;
        }
        Ok(())
    }
}

impl SrcReferrer for StatementList {
    fn src_ref(&self) -> SrcRef {
        if let (Some(first), Some(last)) = (self.iter().next(), self.iter().last()) {
            SrcRef::merge(first, last)
        } else {
            SrcRef(None)
        }
    }
}
