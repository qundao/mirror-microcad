// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

use crate::{CastInto, ir};
use derive_more::Display;
use microcad_lang_base::{Identifiable, Identifier, SrcRef, SrcReferrer};

use serde::{Deserialize, Serialize};

/// NamedArgument in a [`Call`].
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "Expr: Serialize", deserialize = "Expr: Deserialize<'de>"))]
pub struct NamedArgument<Expr> {
    /// Name of the argument
    pub id: Identifier,
    /// Value of the argument
    pub expression: Expr,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<Expr> std::fmt::Display for NamedArgument<Expr>
where
    Expr: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.id, self.expression)
    }
}

impl<Src, Dst> CastInto<NamedArgument<Dst>> for NamedArgument<Src>
where
    Src: CastInto<Dst>,
{
    fn cast_into(self) -> NamedArgument<Dst> {
        NamedArgument {
            id: self.id,
            expression: self.expression.cast_into(),
            src_ref: self.src_ref,
        }
    }
}

impl<Expr> Identifiable for NamedArgument<Expr> {
    fn id_ref(&self) -> &Identifier {
        &self.id
    }
}

impl<Expr> SrcReferrer for NamedArgument<Expr> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "Expr: Serialize", deserialize = "Expr: Deserialize<'de>"))]
pub enum Argument<Expr> {
    /// Explicitly positional: `10` or `x + 1`
    Unnamed(Expr),
    /// Explicitly named by caller: `b: 12`
    Named {
        name: Identifier,
        expr: Expr,
        src_ref: SrcRef,
    },
    /// Auto-bind candidate identifier: `b` (needs resolver to check against ParameterList)
    AutoNamed { name: Identifier, expr: Expr },
}

impl<Expr> SrcReferrer for Argument<Expr>
where
    Expr: SrcReferrer,
{
    fn src_ref(&self) -> SrcRef {
        match self {
            Argument::Unnamed(expr) | Argument::AutoNamed { expr, .. } => expr.src_ref(),
            Argument::Named { src_ref, .. } => *src_ref,
        }
    }
}

impl<Expr> Argument<Expr> {
    pub fn name(&self) -> Option<&Identifier> {
        match self {
            Argument::Unnamed(_) => None,
            Argument::Named { name, .. } | Argument::AutoNamed { name, .. } => Some(name),
        }
    }
}

impl<Src: ir::ExprSpec, Dst: ir::ExprSpec> CastInto<Argument<Dst>> for Argument<Src>
where
    Src: CastInto<Dst>,
    Src::Name: Into<Dst::Name>,
{
    fn cast_into(self) -> Argument<Dst> {
        match self {
            Argument::Unnamed(pos) => Argument::Unnamed(pos.cast_into()),
            Argument::Named {
                name,
                expr,
                src_ref,
            } => Argument::Named {
                name: name.cast_into(),
                expr: expr.cast_into(),
                src_ref,
            },
            Argument::AutoNamed { name, expr } => Argument::AutoNamed {
                name: name.cast_into(),
                expr: expr.cast_into(),
            },
        }
    }
}

impl<Expr> std::fmt::Display for Argument<Expr>
where
    Expr: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Unnamed(expr) => write!(f, "{expr}"),
            Argument::Named { name, expr, .. } => write!(f, "{name} = {expr}"),
            Argument::AutoNamed { name, expr } => write!(f, "~{name} = {expr}"),
        }
    }
}

impl<Expr: ir::ExprSpec> From<Expr> for Argument<Expr> {
    fn from(expr: Expr) -> Self {
        match expr.single_identifier() {
            Some(name) => Self::AutoNamed {
                name: name.clone(),
                expr,
            },
            None => Self::Unnamed(expr),
        }
    }
}

/// *Ordered map* of arguments in a [`Call`].
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "Expr: Serialize", deserialize = "Expr: Deserialize<'de>"))]
pub struct ArgumentList<Expr> {
    /// Source code reference
    pub src_ref: SrcRef,

    /// The unnamed arguments.
    pub args: Box<[Argument<Expr>]>,
}

impl<Expr> ArgumentList<Expr> {
    /// Prepends a positional argument to the front of the argument list.
    pub fn prepend(&mut self, arg: impl Into<Argument<Expr>>) {
        let mut new_args = Vec::with_capacity(self.args.len() + 1);
        new_args.push(arg.into());
        new_args.extend(Vec::from(std::mem::take(&mut self.args)));
        self.args = new_args.into_boxed_slice();
    }

    /// Consumes `self` and returns a new `ArgumentList` with `expr` prepended to `unnamed_args`.
    pub fn prepended(mut self, arg: impl Into<Argument<Expr>>) -> Self {
        self.prepend(arg);
        self
    }
}

impl<Expr> std::fmt::Display for ArgumentList<Expr>
where
    Expr: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        })
    }
}

impl<S, Expr> FromIterator<S> for ArgumentList<Expr>
where
    S: Into<Argument<Expr>>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self {
            src_ref: SrcRef::default(),
            args: iter
                .into_iter()
                .map(|arg| arg.into())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl<Src: ir::ExprSpec, Dst: ir::ExprSpec> CastInto<ArgumentList<Dst>> for ArgumentList<Src>
where
    Src: CastInto<Dst>,
    Src::Name: Into<Dst::Name>,
{
    fn cast_into(self) -> ArgumentList<Dst> {
        ArgumentList {
            src_ref: self.src_ref,
            args: self.args.cast_into(),
        }
    }
}

/// Call of a *workbench* or *function*.
#[derive(Debug, Display, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[display("{}({})", name, args)]
#[serde(bound(
    serialize = "Expr: Serialize, Expr::Name: Serialize",
    deserialize = "Expr: Deserialize<'de>, Expr::Name: Deserialize<'de>"
))]
pub struct Call<Expr: ir::ExprSpec> {
    /// Name of the call.
    pub name: Expr::Name,
    /// Argument list of the call.
    pub args: ir::ArgumentList<Expr>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<Src: ir::ExprSpec, Dst: ir::ExprSpec> CastInto<Call<Dst>> for Call<Src>
where
    Src: CastInto<Dst>,
    Src::Name: Into<Dst::Name>,
{
    fn cast_into(self) -> Call<Dst> {
        Call {
            name: self.name.into(),
            args: self.args.cast_into(),
            src_ref: self.src_ref,
        }
    }
}
