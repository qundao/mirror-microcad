// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! statement syntax elements

use crate::lower::ir;

mod assignment_statement;
mod expression_statement;
mod inner_doc_comment;
mod return_statement;
mod statement_list;

pub use assignment_statement::*;
pub use expression_statement::*;
pub use inner_doc_comment::*;
use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_proc_macros::SrcReferrer;
pub use return_statement::*;
pub use statement_list::*;

use std::rc::Rc;

/// A constant definition: `const FOO: Length = 32mm`.
#[derive(Clone, Debug, SrcReferrer)]
pub struct Constant {
    pub doc: ir::DocBlock,
    pub attr: ir::AttributeList,
    pub visibility: ir::Visibility,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
    pub id: ir::Identifier,
    pub ty: Option<ir::TypeAnnotation>,
    pub expr: ir::ConstantExpression,
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.ty {
            Some(ty) => write!(
                f,
                "{vis}const {id}: {ty} = {expr}",
                vis = self.visibility,
                id = self.id,
                expr = self.expr
            ),
            None => write!(
                f,
                "{vis}const {id} = {expr}",
                vis = self.visibility,
                id = self.id,
                expr = self.expr
            ),
        }
    }
}

/// A property: `prop a: Length = 42mm`.
///
/// TODO: Move to workbench eventually.
#[derive(Clone, Debug, SrcReferrer)]
pub struct PropertyAssignment {
    pub doc: ir::DocBlock,
    pub attr: ir::AttributeList,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
    pub id: ir::Identifier,
    pub ty: Option<ir::TypeAnnotation>,
    pub expr: ir::Expression,
}

impl std::fmt::Display for PropertyAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.ty {
            Some(ty) => write!(
                f,
                "prop {id}: {ty} = {expr}",
                id = self.id,
                expr = self.expr
            ),
            None => write!(f, "prop {id} = {expr}", id = self.id, expr = self.expr),
        }
    }
}

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
    /// Constant definition
    Constant(ir::Constant),

    /// Use statement
    Use(ir::UseStatement),
    /// Return statement
    Return(ir::ReturnStatement),
    /// If statement
    If(ir::If),
    /// Inner attribute statement: `#![size = std::A4]`.
    InnerAttribute(ir::Attribute),
    /// Inner doc comment: `//! Text`.
    InnerDocComment(ir::InnerDocComment),

    /// Property statement
    Property(ir::PropertyAssignment),

    /// Assignment statement.
    LocalAssignment(ir::LocalAssignmentStatement),
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
            Self::Constant(c) => c.src_ref(),

            Self::Use(us) => us.src_ref(),
            Self::Return(r) => r.src_ref(),
            Self::If(i) => i.src_ref(),
            Self::InnerAttribute(i) => i.src_ref(),
            Self::InnerDocComment(i) => i.src_ref(),

            Self::Property(p) => p.src_ref(),
            Self::LocalAssignment(a) => a.src_ref(),
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
            Self::Constant(c) => {
                write!(f, "{c}")
            }

            Self::Use(u) => write!(f, "{u};"),
            Self::Return(r) => write!(f, "{r};"),
            Self::If(i) => write!(f, "{i}"),
            Self::InnerAttribute(i) => write!(f, "{i}"),
            Self::InnerDocComment(i) => write!(f, "{i}"),

            Self::Property(p) => write!(f, "{p}"),
            Self::LocalAssignment(a) => write!(f, "{a}"),
            Self::Expression(e) => write!(f, "{e}"),
        }
    }
}
