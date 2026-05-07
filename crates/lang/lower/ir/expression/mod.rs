// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

use crate::lower::ir;

mod array_expression;
mod marker;
mod range_expression;
mod tuple_expression;

pub use array_expression::*;
pub use marker::*;
pub use range_expression::*;
pub use tuple_expression::*;

use microcad_lang_base::{Identifier, Refer, SrcRef, SrcReferrer};

/// List of expressions.
pub type ListExpression = Vec<Expression>;

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
    If(Box<ir::IfStatement>),
    /// A call: `ops::subtract()`.
    Call(ir::Call),
    /// A qualified name: `foo::bar`.
    QualifiedName(ir::QualifiedName),
    /// A marker expression: `@input`.
    Marker(Marker),
    /// A binary operation: `a + b`
    BinaryOp {
        /// Left-hand side
        lhs: Box<Expression>,
        /// Operator  ('+', '-', '/', '*', '<', '>', '≤', '≥', '&', '|')
        op: Refer<String>,
        /// Right -hand side
        rhs: Box<Expression>,
        /// Source code reference
        src_ref: SrcRef,
    },
    /// A unary operation: !a
    UnaryOp {
        /// Operator ('+', '-', '!')
        op: Refer<String>,
        /// Right -hand side
        rhs: Box<Expression>,
        /// Source code reference
        src_ref: SrcRef,
    },
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
            Expression::BinaryOp {
                lhs,
                op: _,
                rhs,
                src_ref: _,
            } => {
                let l = lhs.single_identifier();
                let r = rhs.single_identifier();
                if l == r || r.is_none() {
                    l
                } else if l.is_none() {
                    r
                } else {
                    None
                }
            }
            Expression::UnaryOp {
                op: _,
                rhs,
                src_ref: _,
            } => rhs.single_identifier(),
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
            Self::BinaryOp {
                lhs: _,
                op: _,
                rhs: _,
                src_ref,
            } => src_ref.clone(),
            Self::UnaryOp {
                op: _,
                rhs: _,
                src_ref,
            } => src_ref.clone(),
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
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => write!(f, "{lhs} {op} {rhs}"),
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => write!(f, "{op}{rhs}"),
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

impl AsRef<Self> for Expression {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::FormatString(l0), Self::FormatString(r0)) => l0 == r0,
            (Self::ArrayExpression(l0), Self::ArrayExpression(r0)) => l0 == r0,
            (Self::TupleExpression(l0), Self::TupleExpression(r0)) => l0 == r0,
            (Self::QualifiedName(l0), Self::QualifiedName(r0)) => l0 == r0,
            (
                Self::BinaryOp {
                    lhs: l_lhs,
                    op: l_op,
                    rhs: l_rhs,
                    src_ref: l_src_ref,
                },
                Self::BinaryOp {
                    lhs: r_lhs,
                    op: r_op,
                    rhs: r_rhs,
                    src_ref: r_src_ref,
                },
            ) => l_lhs == r_lhs && l_op == r_op && l_rhs == r_rhs && l_src_ref == r_src_ref,
            (
                Self::UnaryOp {
                    op: l_op,
                    rhs: l_rhs,
                    src_ref: l_src_ref,
                },
                Self::UnaryOp {
                    op: r_op,
                    rhs: r_rhs,
                    src_ref: r_src_ref,
                },
            ) => l_op == r_op && l_rhs == r_rhs && l_src_ref == r_src_ref,
            (Self::ArrayElementAccess(l0, l1, l2), Self::ArrayElementAccess(r0, r1, r2)) => {
                l0 == r0 && l1 == r1 && l2 == r2
            }
            _ => unreachable!("PartialEq implemented for const expressions only"),
        }
    }
}
