// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod identifier;

pub use identifier::{Identifier, IdentifierList, ShortId};
use serde::{Deserialize, Serialize};
use strum::EnumString;

/// The possible type of workbenches
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Hash, PartialEq, Default, Serialize, Deserialize)]
pub enum Visibility {
    /// `pub`
    Public,
    /// Everything is private by default.
    #[default]
    Private,
}

/// The type of the operator for binary operations
#[derive(Debug, PartialEq, Clone, EnumString, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Clone, EnumString, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum UnaryOperator {
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "!")]
    Not,
}
