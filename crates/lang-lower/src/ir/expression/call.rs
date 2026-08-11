// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

use crate::{CastInto, MakeHumanReadable, Unresolver, ir};
use derive_more::Display;
use microcad_lang_base::{Identifier, SrcRef, SrcReferrer};

use serde::{Deserialize, Serialize};

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

impl<Expr: ir::ExprSpec> MakeHumanReadable for Argument<Expr> {
    fn make_human_readable<U: Unresolver>(&mut self, unresolver: &U) {
        match self {
            Argument::Unnamed(expr)
            | Argument::Named { expr, .. }
            | Argument::AutoNamed { expr, .. } => expr.make_human_readable(unresolver),
        }
    }
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
{
    fn cast_into(self) -> Argument<Dst> {
        match self {
            Argument::Unnamed(pos) => Argument::Unnamed(pos.cast_into()),
            Argument::Named {
                name,
                expr,
                src_ref,
            } => Argument::Named {
                name: name,
                expr: expr.cast_into(),
                src_ref,
            },
            Argument::AutoNamed { name, expr } => Argument::AutoNamed {
                name: name,
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

impl<Expr: ir::ExprSpec> MakeHumanReadable for ArgumentList<Expr> {
    fn make_human_readable<T: Unresolver>(&mut self, unresolver: &T) {
        self.args.make_human_readable(unresolver)
    }
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
#[display("{path}({args})")]
#[serde(bound(serialize = "Expr: Serialize", deserialize = "Expr: Deserialize<'de>"))]
pub struct Call<Expr: ir::ExprSpec> {
    /// Path of the call.
    pub path: ir::Path,
    /// Argument list of the call.
    pub args: ir::ArgumentList<Expr>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<Src: ir::ExprSpec, Dst: ir::ExprSpec> CastInto<Call<Dst>> for Call<Src>
where
    Src: CastInto<Dst>,
{
    fn cast_into(self) -> Call<Dst> {
        Call {
            path: self.path.into(),
            args: self.args.cast_into(),
            src_ref: self.src_ref,
        }
    }
}

impl<Expr: ir::ExprSpec> MakeHumanReadable for Call<Expr> {
    fn make_human_readable<U: Unresolver>(&mut self, unresolver: &U) {
        self.path.make_human_readable(unresolver);
        self.args.make_human_readable(unresolver);
    }
}
