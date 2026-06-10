// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

use crate::{SingleIdentifier, ir};

mod array_expression;
mod call;
mod format_string;
mod literal;
mod qualified_name;
mod range_expression;
mod tuple_expression;

pub use array_expression::*;
pub use call::*;
pub use format_string::*;
pub use literal::*;
pub use qualified_name::*;
pub use range_expression::*;
pub use tuple_expression::*;

use microcad_lang_base::{Identifier, Refer, SrcRef, SrcReferrer};

/// List of expressions.
pub type ListExpression<EXPR> = Vec<EXPR>;

/// If statement.
#[derive(Clone, Debug)]
pub struct If<EXPR, BODY> {
    /// SrcRef of the `if` keyword.
    pub if_ref: SrcRef,
    /// If condition.
    pub cond: Box<EXPR>,
    /// Body if `true`.
    pub body: BODY,
    /// SrcRef of the `else` keyword, if present.
    pub else_ref: Option<SrcRef>,
    /// Body if `false`.
    pub body_else: Option<BODY>,
    /// SrcRef of the `else[ if]` keyword, if present.
    pub next_if_ref: Option<SrcRef>,
    /// Next if statement: `else if x == 1`.
    pub next_if: Option<Box<If<EXPR, BODY>>>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<EXPR, BODY> std::fmt::Display for If<EXPR, BODY>
where
    EXPR: std::fmt::Display,
    BODY: std::fmt::Display,
{
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

/// A binary operation: `a + b`
#[derive(Clone, Debug)]
pub struct BinaryOp<EXPR> {
    /// Left-hand side
    pub lhs: Box<EXPR>,
    /// Operator  ('+', '-', '/', '*', '<', '>', '≤', '≥', '&', '|')
    pub op: Refer<String>,
    /// Right -hand side
    pub rhs: Box<EXPR>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<EXPR> SrcReferrer for BinaryOp<EXPR> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl<EXPR> SingleIdentifier for BinaryOp<EXPR>
where
    EXPR: SingleIdentifier,
{
    fn single_identifier(&self) -> Option<&Identifier> {
        match (self.lhs.single_identifier(), self.rhs.single_identifier()) {
            (None, Some(lhs)) => Some(lhs),
            (Some(rhs), None) => Some(rhs),
            (Some(lhs), Some(rhs)) => {
                if lhs == rhs {
                    Some(lhs)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl<EXPR> std::fmt::Display for BinaryOp<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{lhs} {op} {rhs}",
            lhs = self.lhs,
            op = self.op,
            rhs = self.rhs
        )
    }
}

/// A unary operation: !a
#[derive(Clone, Debug)]
pub struct UnaryOp<EXPR> {
    /// Operator ('+', '-', '!')
    pub op: Refer<String>,
    /// Right -hand side
    pub rhs: Box<EXPR>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<EXPR> SrcReferrer for UnaryOp<EXPR> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl<EXPR> SingleIdentifier for UnaryOp<EXPR>
where
    EXPR: SingleIdentifier,
{
    fn single_identifier(&self) -> Option<&Identifier> {
        self.rhs.single_identifier()
    }
}

impl<EXPR> std::fmt::Display for UnaryOp<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{op}{rhs}", op = self.op, rhs = self.rhs)
    }
}

#[derive(Debug)]
pub struct ElementAccess<EXPR, ELEMENT> {
    pub lhs: Box<EXPR>,
    pub element: ELEMENT,
    pub src_ref: SrcRef,
}

/// An expression that can be evaluated during `resolve` phase.
///
/// Use for `Constant` and default values for `Parameter`.
/// TODO: ElementAccess are missing.
#[derive(Debug, derive_more::From)]
pub enum ConstantExpression {
    Invalid,
    Literal(ir::Literal),
    Call(ir::Call<ConstantExpression>),
    QualifiedName(ir::QualifiedName),
    FormatString(ir::FormatString),
    ArrayExpression(ir::ArrayExpression<ConstantExpression>),
    TupleExpression(ir::TupleExpression<ConstantExpression>),
    BinaryOp(ir::BinaryOp<ConstantExpression>),
    UnaryOp(ir::UnaryOp<ConstantExpression>),
}

impl std::fmt::Display for ConstantExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ConstantExpression::Literal(literal) => write!(f, "{literal}"),
            ConstantExpression::Call(call) => write!(f, "{call}"),
            ConstantExpression::QualifiedName(qualified_name) => write!(f, "{qualified_name}"),
            ConstantExpression::FormatString(format_string) => write!(f, "{format_string}"),
            ConstantExpression::ArrayExpression(array_expression) => {
                write!(f, "{array_expression}")
            }
            ConstantExpression::TupleExpression(tuple_expression) => {
                write!(f, "{tuple_expression}")
            }
            ConstantExpression::BinaryOp(binary_op) => write!(f, "{binary_op}"),
            ConstantExpression::UnaryOp(unary_op) => write!(f, "{unary_op}"),
            _ => unimplemented!(),
        }
    }
}
