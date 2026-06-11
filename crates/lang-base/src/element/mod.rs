// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod identifier;

pub use identifier::{Identifier, IdentifierList, ShortId};
use serde::Serialize;
use strum::EnumString;

use crate::{Refer, SrcRef};

/// The possible type of workbenches
#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
pub enum WorkbenchKind {
    /// `sketch`
    Sketch,
    /// `part`
    Part,
    /// `op`
    Op,
}

impl std::fmt::Display for WorkbenchKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                WorkbenchKind::Sketch => "sketch",
                WorkbenchKind::Part => "part",
                WorkbenchKind::Op => "op",
            }
        )
    }
}

/// An optional visibility modifier
///
/// it can be part of constant, module, function or workbench definitions.
#[derive(Debug, PartialEq, Default)]
pub enum Visibility {
    /// `pub`
    Public,
    /// Everything is private by default.
    #[default]
    Private,
}

/// Lines of inner or outer doc block including prefix `///`/`//!`.
#[derive(Debug, Default, PartialEq)]
pub struct DocBlock<REF = SrcRef> {
    pub src_ref: REF,
    pub lines: Vec<String>,
}

/// A binary operation
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct BinaryOperation<EXPR> {
    pub src_ref: SrcRef,
    pub lhs: Box<EXPR>,
    pub op: Refer<BinaryOperator>,
    pub rhs: Box<EXPR>,
}

/// The type of the operator for binary operations
#[derive(Debug, PartialEq, Clone, EnumString)]
pub enum BinaryOperator {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "-")]
    Subtract,
    #[strum(serialize = "*")]
    Multiply,
    #[strum(serialize = "/")]
    Divide,
    #[strum(serialize = "|")]
    Union,
    #[strum(serialize = "&")]
    Intersect,
    #[strum(serialize = "^")]
    PowerXor,
    #[strum(serialize = ">")]
    GreaterThan,
    #[strum(serialize = "<")]
    LessThan,
    #[strum(serialize = ">=", serialize = "≥")]
    GreaterEqual,
    #[strum(serialize = "<=", serialize = "≤")]
    LessEqual,
    #[strum(serialize = "==")]
    Equal,
    #[strum(serialize = "~=")]
    Near,
    #[strum(serialize = "!=")]
    NotEqual,
    #[strum(serialize = "and")]
    And,
    #[strum(serialize = "or")]
    Or,
    #[strum(serialize = "xor")]
    Xor,
}

/// The type of the operator for unary operations
#[derive(Debug, PartialEq, Clone, EnumString)]
#[allow(missing_docs)]
pub enum UnaryOperator {
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "!")]
    Not,
}

/// A unary operation
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct UnaryOperation<EXPR> {
    pub op: UnaryOperator,
    pub rhs: Box<EXPR>,
}
