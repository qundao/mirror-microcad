// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

use crate::lower::{SingleIdentifier, ir};

mod array_expression;
mod marker;
mod range_expression;
mod tuple_expression;

pub use array_expression::*;
pub use marker::*;
pub use range_expression::*;
pub use tuple_expression::*;

use microcad_lang_base::{Identifier, Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::SrcReferrer;

/// List of expressions.
pub type ListExpression<EXPR = ir::Expression> = Vec<EXPR>;

/// If statement.
#[derive(Clone, Debug, SrcReferrer)]
pub struct If<BODY = ir::Body> {
    /// SrcRef of the `if` keyword.
    pub if_ref: SrcRef,
    /// If condition.
    pub cond: ir::Expression,
    /// Body if `true`.
    pub body: BODY,
    /// SrcRef of the `else` keyword, if present.
    pub else_ref: Option<SrcRef>,
    /// Body if `false`.
    pub body_else: Option<BODY>,
    /// SrcRef of the `else[ if]` keyword, if present.
    pub next_if_ref: Option<SrcRef>,
    /// Next if statement: `else if x == 1`.
    pub next_if: Option<Box<If<BODY>>>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for If {
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

/// Any expression.
#[derive(Clone, Debug, Default)]
pub enum Expression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,
    /// An integer, float, color or bool literal: 1, 1.0, #00FF00, false
    Literal(ir::Literal),
    /// A string that contains format expressions: "value = {a}"
    FormatString(ir::FormatString),
    /// A list: [a, b, c]
    ArrayExpression(ArrayExpression),
    /// A tuple: (a, b, c)
    TupleExpression(TupleExpression),
    /// A body: `{}`.
    Body(ir::Body),
    /// An if statement: `if {} else {}`.
    If(Box<ir::If<ir::Body>>),
    /// A call: `ops::subtract()`.
    Call(ir::Call),
    /// A qualified name: `foo::bar`.
    QualifiedName(ir::QualifiedName),
    /// A marker expression: `@input`.
    Marker(Marker),
    /// A binary operation: `a + b`
    BinaryOp(BinaryOp),
    /// A unary operation: !a
    UnaryOp(UnaryOp),
    /// Access an element of a list (`a[0]`) or a tuple (`a.0` or `a.b`)
    ArrayElementAccess(Box<Expression>, Box<Expression>, SrcRef),
    /// Access an element of a tuple: `a.b`.
    PropertyAccess(Box<Expression>, Identifier, SrcRef),

    /// Access an attribute of a model: `a#b`.
    AttributeAccess(Box<Expression>, Identifier, SrcRef),

    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to a value
    MethodCall(Box<Expression>, ir::MethodCall, SrcRef),
}

/// A binary operation: `a + b`
#[derive(Clone, Debug)]
pub struct BinaryOp<EXPR = ir::Expression> {
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

impl SingleIdentifier for BinaryOp {
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
pub struct UnaryOp<EXPR = ir::Expression> {
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

impl SingleIdentifier for UnaryOp {
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

/// An expression that can be evaluated during `resolve` phase.
///
/// Use for `Constant` and default values for `Parameter`.
/// TODO: ElementAccess are missing.
#[derive(Debug, Clone, derive_more::From)]
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

impl crate::lower::SingleIdentifier for Expression {
    /// If the expression includes just one identifier, e.g. `a` or `a * (a + 2)`
    fn single_identifier(&self) -> Option<&Identifier> {
        match &self {
            Expression::Invalid
            | Expression::Literal(..)
            | Expression::FormatString(..)
            | Expression::Marker(..)
            | Expression::PropertyAccess(..)
            | Expression::AttributeAccess(..)
            | Expression::MethodCall(..)
            | Expression::ArrayExpression(..)
            | Expression::TupleExpression(..)
            | Expression::Body(..)
            | Expression::If(..)
            | Expression::Call(..) => None,

            Expression::QualifiedName(qualified_name) => qualified_name.single_identifier(),
            Expression::BinaryOp(binary_op) => binary_op.single_identifier(),
            Expression::UnaryOp(unary_op) => unary_op.single_identifier(),
            Expression::ArrayElementAccess(expression, ..) => expression.single_identifier(),
        }
    }
}

impl SrcReferrer for Expression {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Invalid => SrcRef::none(),
            Self::Literal(l) => l.src_ref(),
            Self::FormatString(fs) => fs.src_ref(),
            Self::ArrayExpression(le) => le.src_ref(),
            Self::TupleExpression(te) => te.src_ref(),
            Self::Call(c) => c.src_ref(),
            Self::Body(b) => b.src_ref(),
            Self::If(i) => i.src_ref(),
            Self::QualifiedName(q) => q.src_ref(),
            Self::Marker(m) => m.src_ref(),
            Self::BinaryOp(binary_op) => binary_op.src_ref(),
            Self::UnaryOp(unary_op) => unary_op.src_ref(),
            Self::ArrayElementAccess(_, _, src_ref) => src_ref.clone(),
            Self::PropertyAccess(_, _, src_ref) => src_ref.clone(),
            Self::AttributeAccess(_, _, src_ref) => src_ref.clone(),
            Self::MethodCall(_, _, src_ref) => src_ref.clone(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::FormatString(format_string) => write!(f, "{format_string}"),
            Self::ArrayExpression(array_expression) => write!(f, "{array_expression}"),
            Self::TupleExpression(tuple_expression) => write!(f, "{tuple_expression}"),
            Self::BinaryOp(binary_op) => write!(f, "{binary_op}"),
            Self::UnaryOp(unary_op) => write!(f, "{unary_op}"),
            Self::ArrayElementAccess(lhs, rhs, _) => write!(f, "{lhs}[{rhs}]"),
            Self::PropertyAccess(lhs, rhs, _) => write!(f, "{lhs}.{rhs}"),
            Self::AttributeAccess(lhs, rhs, _) => write!(f, "{lhs}#{rhs}"),
            Self::MethodCall(lhs, method_call, _) => write!(f, "{lhs}.{method_call}"),
            Self::Call(call) => write!(f, "{call}"),
            Self::Body(body) => write!(f, "{body}"),
            Self::If(if_) => write!(f, "{if_}"),
            Self::QualifiedName(qualified_name) => write!(f, "{qualified_name}"),
            Self::Marker(marker) => write!(f, "{marker}"),
            _ => unimplemented!(),
        }
    }
}
