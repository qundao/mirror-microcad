// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! statement syntax elements

use crate::lower::ir;

mod assignment_statement;
mod expression_statement;
mod if_statement;
mod inner_doc_comment;
mod return_statement;
mod statement_list;

pub use assignment_statement::*;
pub use expression_statement::*;
pub use if_statement::*;
pub use inner_doc_comment::*;
use microcad_lang_base::{SrcRef, SrcReferrer};
pub use return_statement::*;
pub use statement_list::*;

use std::rc::Rc;

/// Any statement.
#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum Statement {
    /// Part definition
    Workbench(Rc<ir::WorkbenchDefinition>),
    /// Module definition
    Module(Rc<ir::ModuleDefinition>),
    /// Function definition
    Function(Rc<ir::FunctionDefinition>),
    /// Init definition
    Init(Rc<ir::InitDefinition>),

    /// Use statement
    Use(ir::UseStatement),
    /// Return statement
    Return(ir::ReturnStatement),
    /// If statement
    If(ir::IfStatement),
    /// Inner attribute statement: `#![size = std::A4]`.
    InnerAttribute(ir::Attribute),
    /// Inner doc comment: `//! Text`.
    InnerDocComment(ir::InnerDocComment),

    /// Assignment statement.
    Assignment(ir::AssignmentStatement),
    /// Expression statement.
    Expression(ir::ExpressionStatement),
}

impl SrcReferrer for Statement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Workbench(w) => w.src_ref(),
            Self::Module(m) => m.src_ref(),
            Self::Function(fd) => fd.src_ref(),
            Self::Init(mid) => mid.src_ref(),

            Self::Use(us) => us.src_ref(),
            Self::Return(r) => r.src_ref(),
            Self::If(i) => i.src_ref(),
            Self::InnerAttribute(i) => i.src_ref(),
            Self::InnerDocComment(i) => i.src_ref(),

            Self::Assignment(a) => a.src_ref(),
            Self::Expression(e) => e.src_ref(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::lower::Identifiable;
        match self {
            Self::Workbench(w) => {
                write!(f, "{w}")
            }
            Self::Module(m) => {
                write!(f, "{}", m.id_ref())
            }
            Self::Function(f_) => {
                write!(f, "{}", f_.id_ref())
            }
            Self::Init(mi) => {
                write!(f, "{mi}")
            }

            Self::Use(u) => write!(f, "{u};"),
            Self::Return(r) => write!(f, "{r};"),
            Self::If(i) => write!(f, "{i}"),
            Self::InnerAttribute(i) => write!(f, "{i}"),
            Self::InnerDocComment(i) => write!(f, "{i}"),

            Self::Assignment(a) => write!(f, "{a}"),
            Self::Expression(e) => write!(f, "{e}"),
        }
    }
}
