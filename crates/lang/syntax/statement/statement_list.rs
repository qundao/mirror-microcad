// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Statement list syntax element.

use crate::{src_ref::*, syntax::*};

/// A list of statements.
#[derive(Clone, Default)]
pub struct StatementList {
    /// Statements (terminated with `;`)
    pub statements: Vec<Statement>,
    /// Tail expression
    pub tail: Option<Box<Statement>>,
}

impl StatementList {
    /// Get iterator over all statements (including tail)
    pub fn iter(&self) -> impl Iterator<Item = &Statement> {
        self.statements.iter().chain(self.tail.as_deref())
    }

    /// return number of statements (including tail)
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// return true if statements (including tail) are empty
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty() && self.tail.is_none()
    }

    /// get expression out of tail
    pub fn tail_expression(&self) -> Option<&Expression> {
        if let Some(tail) = &self.tail.as_deref() {
            match tail {
                Statement::Expression(exp) => Some(&exp.expression),
                _ => unreachable!("unexpected tail"),
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for StatementList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            writeln!(f, "{statement}")?;
        }
        self.tail_expression()
            .map(|exp| writeln!(f, "{exp}"))
            .unwrap_or(Ok(()))?;
        Ok(())
    }
}

impl std::fmt::Debug for StatementList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            writeln!(f, "{statement:?}")?;
        }
        if let Some(tail) = &self.tail.as_deref() {
            match tail {
                Statement::Expression(exp) => writeln!(f, "{:?}", exp.expression)?,
                _ => unreachable!("unexpected tail"),
            }
        }
        Ok(())
    }
}

impl TreeDisplay for StatementList {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        if !self.statements.is_empty() {
            writeln!(f, "{:depth$}Statements:", "")?;
            self.statements
                .iter()
                .try_for_each(|s| s.tree_print(f, depth.indented()))?;
        }
        if let Some(exp) = self.tail_expression() {
            writeln!(f, "{:depth$}Tail:", "")?;
            exp.tree_print(f, depth.indented())?;
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
