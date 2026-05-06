// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement syntax elements.

use microcad_lang_base::{SrcRef, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

use crate::lower::ir;

/// If statement.
#[derive(Clone, Debug, SrcReferrer)]
pub struct IfStatement {
    /// SrcRef of the `if` keyword.
    pub if_ref: SrcRef,
    /// If condition.
    pub cond: ir::Expression,
    /// Body if `true`.
    pub body: ir::Body,
    /// SrcRef of the `else` keyword, if present.
    pub else_ref: Option<SrcRef>,
    /// Body if `false`.
    pub body_else: Option<ir::Body>,
    /// SrcRef of the `else[ if]` keyword, if present.
    pub next_if_ref: Option<SrcRef>,
    /// Next if statement: `else if x == 1`.
    pub next_if: Option<Box<IfStatement>>,
    /// Source code reference.
    pub src_ref: SrcRef,
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
