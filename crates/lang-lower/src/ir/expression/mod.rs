// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

mod array_expression;
mod call;
mod format_string;
mod literal;
mod range_expression;
mod symbol_path;
mod tuple_expression;

pub use array_expression::*;
pub use call::*;
use derive_more::Display;
pub use format_string::*;
pub use literal::*;
pub use range_expression::*;
pub use symbol_path::*;
pub use tuple_expression::*;

use crate::{CastInto, ir};
use microcad_lang_base::{
    Identifier, SingleIdentifier, SrcRef, SrcReferrer, element::BinaryOperator,
    element::UnaryOperator,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of expressions.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub struct ListExpression<EXPR>(pub Vec<EXPR>);

impl<EXPR> std::fmt::Display for ListExpression<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

impl<T, EXPR> CastInto<ListExpression<T>> for ListExpression<EXPR>
where
    EXPR: CastInto<T> + Serialize,
{
    fn cast_into(self) -> ListExpression<T> {
        ListExpression(self.0.into_iter().map(|e| e.cast_into()).collect())
    }
}

/// If statement.
#[skip_serializing_none]
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "EXPR: Serialize, EXPR::Body: Serialize",
    deserialize = "EXPR: Deserialize<'de>, EXPR::Body: Deserialize<'de>"
))]
pub struct If<EXPR: ExpressionKind> {
    /// SrcRef of the `if` keyword.
    pub if_ref: SrcRef,
    /// If condition.
    pub cond: Box<EXPR>,
    /// Body if `true`.
    pub body: Box<EXPR::Body>,
    /// SrcRef of the `else` keyword, if present.
    pub else_ref: Option<SrcRef>,
    /// Body if `false`.
    pub body_else: Option<Box<EXPR::Body>>,
    /// SrcRef of the `else[ if]` keyword, if present.
    pub next_if_ref: Option<SrcRef>,
    /// Next if statement: `else if x == 1`.
    pub next_if: Option<Box<If<EXPR>>>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<T: ExpressionKind, EXPR: ExpressionKind> CastInto<If<T>> for If<EXPR>
where
    EXPR: CastInto<T>,
    EXPR::Body: CastInto<T::Body> + Serialize,
{
    fn cast_into(self) -> If<T> {
        If {
            if_ref: self.if_ref,
            cond: Box::new(self.cond.cast_into()),
            body: Box::new(self.body.cast_into()),
            else_ref: self.else_ref,
            body_else: self.body_else.map(|body| Box::new(body.cast_into())),
            next_if_ref: self.next_if_ref,
            next_if: self.next_if.map(|next_if| Box::new(next_if.cast_into())),
            src_ref: self.src_ref,
        }
    }
}

impl<EXPR> std::fmt::Display for If<EXPR>
where
    EXPR: ExpressionKind + std::fmt::Display,
    EXPR::Body: std::fmt::Display,
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
#[derive(Clone, Debug, Display, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
#[display("{lhs} {op} {rhs}")]
pub struct BinaryOp<EXPR> {
    /// Left-hand side
    pub lhs: Box<EXPR>,
    /// Operator  ('+', '-', '/', '*', '<', '>', '≤', '≥', '&', '|')
    pub op: BinaryOperator,
    /// Right -hand side
    pub rhs: Box<EXPR>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<T: Serialize, EXPR: Serialize> CastInto<BinaryOp<T>> for BinaryOp<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> BinaryOp<T> {
        BinaryOp {
            lhs: Box::new(self.lhs.cast_into()),
            op: self.op,
            rhs: Box::new(self.rhs.cast_into()),
            src_ref: self.src_ref,
        }
    }
}

impl<EXPR> SrcReferrer for BinaryOp<EXPR> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref
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

/// A unary operation: !a
#[derive(Clone, Display, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
#[display("{op}{rhs}")]
pub struct UnaryOp<EXPR> {
    /// Operator ('+', '-', '!')
    pub op: UnaryOperator,
    /// Right -hand side
    pub rhs: Box<EXPR>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<T: Serialize, EXPR: Serialize> CastInto<UnaryOp<T>> for UnaryOp<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> UnaryOp<T> {
        UnaryOp {
            op: self.op,
            rhs: Box::new(self.rhs.cast_into()),
            src_ref: self.src_ref,
        }
    }
}

impl<EXPR> SrcReferrer for UnaryOp<EXPR> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref
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

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "EXPR: Serialize, ELEMENT: Serialize",
    deserialize = "EXPR: Deserialize<'de>, ELEMENT: Deserialize<'de>"
))]
pub struct ElementAccess<EXPR, ELEMENT> {
    pub lhs: Box<EXPR>,
    pub element: Box<ELEMENT>,
    pub src_ref: SrcRef,
}

impl<A: Serialize, B: Serialize, ElementA: Serialize, ElementB: Serialize>
    CastInto<ElementAccess<A, ElementA>> for ElementAccess<B, ElementB>
where
    B: CastInto<A>,
    ElementB: CastInto<ElementA>,
{
    fn cast_into(self) -> ElementAccess<A, ElementA> {
        ElementAccess {
            lhs: Box::new(self.lhs.cast_into()),
            element: Box::new(self.element.cast_into()),
            src_ref: self.src_ref,
        }
    }
}

pub trait ExpressionKind: Serialize {
    type Name;
    type Body;
}

/// An expression that can be evaluated during `resolve` phase.
///
/// Use for `Constant` and default values for `Parameter`.
/// TODO: ElementAccess are missing.
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "NAME: Serialize", deserialize = "NAME: Deserialize<'de>"))]
pub enum ConstantExpression<NAME: Serialize = ir::SymbolPath> {
    Invalid,
    Literal(ir::Literal),
    Name(NAME),
    FormatString(ir::FormatString<NAME>),
    ArrayExpression(ir::ArrayExpression<ConstantExpression<NAME>>),
    TupleExpression(ir::TupleExpression<ConstantExpression<NAME>>),
    BinaryOp(ir::BinaryOp<ConstantExpression<NAME>>),
    UnaryOp(ir::UnaryOp<ConstantExpression<NAME>>),
}

impl<Name: Serialize> ExpressionKind for ConstantExpression<Name> {
    type Name = Name;
    type Body = (); // Constant expressions have no body.
}

impl<T: Serialize, Name: Serialize> CastInto<ConstantExpression<T>> for ConstantExpression<Name>
where
    Name: Into<T>,
{
    fn cast_into(self) -> ConstantExpression<T> {
        use ConstantExpression::*;
        match self {
            Invalid => Invalid,
            Literal(literal) => Literal(literal),
            Name(name) => Name(name.into()),
            FormatString(format_string) => FormatString(format_string.cast_into()),
            ArrayExpression(array_expression) => ArrayExpression(array_expression.cast_into()),
            TupleExpression(tuple_expression) => TupleExpression(tuple_expression.cast_into()),
            BinaryOp(binary_op) => BinaryOp(binary_op.cast_into()),
            UnaryOp(unary_op) => UnaryOp(unary_op.cast_into()),
        }
    }
}

impl<NAME: Serialize> std::fmt::Display for ConstantExpression<NAME>
where
    NAME: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ConstantExpression::Literal(literal) => write!(f, "{literal}"),
            ConstantExpression::Name(qualified_name) => write!(f, "{qualified_name}"),
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
