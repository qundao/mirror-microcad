// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod identifier;

pub use identifier::{Case, Identifier, IdentifierList, ShortId};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// The possible type of workbenches
#[derive(Debug, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
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

/// Visibility of a symbol.
///
/// This is used to determine if an entity is public or private.
/// By default, entities are private.
#[derive(Debug, Hash, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Visibility {
    /// `pub`
    Public,
    /// Everything is private by default.
    #[default]
    Private,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Private => Ok(()),
            Visibility::Public => write!(f, "pub "),
        }
    }
}

/// The type of the operator for binary operations
#[derive(Debug, PartialEq, Hash, Clone, Copy, EnumString, Display, Serialize, Deserialize)]
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

impl BinaryOperator {
    /// Returns the lower snake_case standard library function name for this operator.
    pub const fn builtin_fn_name(&self) -> &'static str {
        match self {
            Self::Add => "__mu::core::add",
            Self::Subtract => "__mu::core::sub",
            Self::Multiply => "__mu::core::mul",
            Self::Divide => "__mu::core::div",
            Self::Union => "__mu::core::union",
            Self::Intersect => "__mu::core::intersect",
            Self::PowerXor => "__mu::core::power",
            Self::GreaterThan => "__mu::core::greater_than",
            Self::LessThan => "__mu::core::less_than",
            Self::GreaterEqual => "__mu::core::greater_equal",
            Self::LessEqual => "__mu::core::less_equal",
            Self::Equal => "__mu::core::equal",
            Self::Near => "__mu::core::near",
            Self::NotEqual => "__mu::core::not_equal",
            Self::And => "__mu::core::and",
            Self::Or => "__mu::core::or",
            Self::Xor => "__mu::core::xor",
        }
    }
}

/// The type of the operator for unary operations
#[derive(Debug, PartialEq, Hash, Clone, Copy, Display, EnumString, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum UnaryOperator {
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "!")]
    Not,
}

impl UnaryOperator {
    /// Returns the lower snake_case standard library function name for this operator.
    pub const fn builtin_fn_name(&self) -> &'static str {
        match self {
            Self::Minus => "__mu::core::neg",
            Self::Plus => "__mu::core::plus",
            Self::Not => "__mu::core::not",
        }
    }
}
