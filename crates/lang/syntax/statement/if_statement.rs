// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement syntax elements.

use microcad_lang_base::{SrcRef, SrcReferrer, TreeDisplay, TreeState};

use crate::syntax::*;

/// If statement.
#[derive(Clone, Debug)]
pub struct IfStatement {
    /// SrcRef of the `if` keyword.
    pub if_ref: SrcRef,
    /// If condition.
    pub cond: Expression,
    /// Body if `true`.
    pub body: Body,
    /// SrcRef of the `else` keyword, if present.
    pub else_ref: Option<SrcRef>,
    /// Body if `false`.
    pub body_else: Option<Body>,
    /// SrcRef of the `else[ if]` keyword, if present.
    pub next_if_ref: Option<SrcRef>,
    /// Next if statement: `else if x == 1`.
    pub next_if: Option<Box<IfStatement>>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl IfStatement {
    /// Checks if all branches of the if statement are set
    pub fn is_complete(&self) -> bool {
        if let Some(next_if) = &self.next_if {
            next_if.is_complete()
        } else {
            self.body_else.is_some()
        }
    }
}

impl SrcReferrer for IfStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "if {cond} {body}", cond = self.cond, body = self.body)?;
        if let Some(next) = &self.next_if {
            writeln!(f, "else {next}")?;
        }
        if let Some(body) = &self.body_else {
            writeln!(f, "else {body}")?;
        }
        Ok(())
    }
}

impl TreeDisplay for IfStatement {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}IfStatement:", "")?;
        depth.indent();
        writeln!(f, "{:depth$}Condition:", "")?;
        self.cond.tree_print(f, depth.indented())?;
        writeln!(f, "{:depth$}If:", "")?;
        self.body.tree_print(f, depth.indented())?;
        if let Some(body_else) = &self.body_else {
            writeln!(f, "{:depth$}Else:", "")?;
            body_else.tree_print(f, depth.indented())?;
        }
        Ok(())
    }
}
