// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

use crate::{CastInto, ir};
use derive_more::Display;
use microcad_lang_base::{Identifiable, Identifier, SrcRef, SrcReferrer};

use serde::{Deserialize, Serialize};

/// NamedArgument in a [`Call`].
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub struct NamedArgument<EXPR> {
    /// Name of the argument
    pub id: Identifier,
    /// Value of the argument
    pub expression: EXPR,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<EXPR> std::fmt::Display for NamedArgument<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.id, self.expression)
    }
}

impl<T, EXPR> CastInto<NamedArgument<T>> for NamedArgument<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> NamedArgument<T> {
        NamedArgument {
            id: self.id,
            expression: self.expression.cast_into(),
            src_ref: self.src_ref,
        }
    }
}

impl<EXPR> Identifiable for NamedArgument<EXPR> {
    fn id_ref(&self) -> &Identifier {
        &self.id
    }
}

impl<EXPR> SrcReferrer for NamedArgument<EXPR> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref
    }
}

/// *Ordered map* of arguments in a [`Call`].
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "Expr: Serialize", deserialize = "Expr: Deserialize<'de>"))]
pub struct ArgumentList<Expr> {
    /// Source code reference
    pub src_ref: SrcRef,

    /// The unnamed arguments.
    pub unnamed_args: Box<[Expr]>,
    /// Named arguments, sorted by name.
    pub named_args: Box<[ir::NamedArgument<Expr>]>,
}

impl<Expr> ArgumentList<Expr> {
    /// Prepends a positional argument to the front of the argument list.
    pub fn prepend(&mut self, expression: Expr) {
        let mut new_args = Vec::with_capacity(self.unnamed_args.len() + 1);
        new_args.push(expression);
        new_args.extend(Vec::from(std::mem::take(&mut self.unnamed_args)));
        self.unnamed_args = new_args.into_boxed_slice();
    }

    /// Consumes `self` and returns a new `ArgumentList` with `expr` prepended to `unnamed_args`.
    pub fn prepended(mut self, expr: Expr) -> Self {
        self.prepend(expr);
        self
    }
}

impl<EXPR> std::fmt::Display for ArgumentList<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            self.unnamed_args
                .iter()
                .map(|p| p.to_string())
                .chain(self.named_args.iter().map(|p| p.to_string()))
                .collect::<Vec<_>>()
                .join(", ")
        })
    }
}

impl<Expr> FromIterator<Expr> for ArgumentList<Expr>
where
    Expr: SrcReferrer,
{
    fn from_iter<T: IntoIterator<Item = Expr>>(iter: T) -> Self {
        let unnamed_args = iter.into_iter().collect::<Vec<_>>().into_boxed_slice();

        Self {
            src_ref: SrcRef::default(),
            unnamed_args,
            named_args: Box::new([]),
        }
    }
}

impl<T, EXPR> CastInto<ArgumentList<T>> for ArgumentList<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> ArgumentList<T> {
        ArgumentList {
            src_ref: self.src_ref,
            unnamed_args: self.unnamed_args.cast_into(),
            named_args: self.named_args.cast_into(),
        }
    }
}

/// Call of a *workbench* or *function*.
#[derive(Debug, Display, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[display("{}({})", name, args)]
#[serde(bound(
    serialize = "EXPR: Serialize, EXPR::Name: Serialize",
    deserialize = "EXPR: Deserialize<'de>, EXPR::Name: Deserialize<'de>"
))]
pub struct Call<EXPR: ir::ExpressionKind> {
    /// Name of the call.
    pub name: EXPR::Name,
    /// Argument list of the call.
    pub args: ir::ArgumentList<EXPR>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<T: ir::ExpressionKind, EXPR: ir::ExpressionKind> CastInto<Call<T>> for Call<EXPR>
where
    EXPR: crate::CastInto<T>,
    EXPR::Name: Into<T::Name>,
{
    fn cast_into(self) -> Call<T> {
        Call {
            name: self.name.into(),
            args: self.args.cast_into(),
            src_ref: self.src_ref,
        }
    }
}
