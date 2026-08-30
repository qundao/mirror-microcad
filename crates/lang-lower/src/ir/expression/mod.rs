// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

mod call;
mod literal;

pub use call::*;
use derive_more::From;
pub use literal::*;
use microcad_lang_types::Value;

use crate::{CastInto, ir};
use microcad_lang_base::{SingleIdentifier, SrcRef, SrcReferrer};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// If statement.
#[skip_serializing_none]
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "Expr: Serialize, Expr::Body: Serialize",
    deserialize = "Expr: Deserialize<'de>, Expr::Body: Deserialize<'de>"
))]
pub struct If<Expr: ExprSpec> {
    /// SrcRef of the `if` keyword.
    pub if_ref: SrcRef,
    /// If condition.
    pub cond: Box<Expr>,
    /// Body if `true`.
    pub body: Box<Expr::Body>,
    /// SrcRef of the `else` keyword, if present.
    pub else_ref: Option<SrcRef>,
    /// Body if `false`.
    pub body_else: Option<Box<Expr::Body>>,
    /// Next if statement: `else if x == 1`.
    pub next_if: Option<Box<If<Expr>>>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<Src: ExprSpec, Dst: ExprSpec> CastInto<If<Dst>> for If<Src>
where
    Src: CastInto<Dst>,
    Src::Body: CastInto<Dst::Body> + Serialize,
{
    fn cast_into(self) -> If<Dst> {
        If {
            if_ref: self.if_ref,
            cond: Box::new(self.cond.cast_into()),
            body: Box::new(self.body.cast_into()),
            else_ref: self.else_ref,
            body_else: self.body_else.map(|body| Box::new(body.cast_into())),
            next_if: self.next_if.map(|next_if| Box::new(next_if.cast_into())),
            src_ref: self.src_ref,
        }
    }
}

impl<Expr> std::fmt::Display for If<Expr>
where
    Expr: ExprSpec + std::fmt::Display,
    Expr::Body: std::fmt::Display,
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

/// Specification trait for an expression.
pub trait ExprSpec: Serialize + SrcReferrer + SingleIdentifier {
    type Body;

    /// Test if an expression holds a constant value.
    fn value(&self) -> Option<&Value>;
}

/// An expression that can be evaluated during `resolve` phase.
#[derive(Debug, Clone, From, PartialEq, Hash, Serialize, Deserialize)]

pub enum ConstantExpression {
    Invalid,
    Value(ir::ConstantValue),
    Path(ir::Path),
    Call(ir::Call<ConstantExpression>),
}

impl SingleIdentifier for ConstantExpression {
    fn single_identifier(&self) -> Option<&microcad_lang_base::Identifier> {
        match self {
            ConstantExpression::Path(name) => name.single_identifier(),
            _ => None,
        }
    }
}

impl SrcReferrer for ConstantExpression {
    fn src_ref(&self) -> SrcRef {
        match &self {
            ConstantExpression::Invalid => SrcRef::none(),
            ConstantExpression::Value(literal) => literal.src_ref(),
            ConstantExpression::Path(name) => name.src_ref(),
            ConstantExpression::Call(call) => call.src_ref,
        }
    }
}

impl ExprSpec for ConstantExpression {
    type Body = (); // Constant expressions have no body.

    fn value(&self) -> Option<&Value> {
        match &self {
            ConstantExpression::Value(constant) => Some(constant.value()),
            _ => None,
        }
    }
}

impl std::fmt::Display for ConstantExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ConstantExpression::Value(literal) => write!(f, "{literal}"),
            ConstantExpression::Path(path) => write!(f, "{path}"),
            _ => unimplemented!(),
        }
    }
}
