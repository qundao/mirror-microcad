// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

use crate::{CastInto, ir};
use microcad_lang_base::{Identifiable, Identifier, SrcRef, SrcReferrer, is_default};

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

/// Unnamed argument in a [`Call`].
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub struct UnnamedArgument<EXPR> {
    /// Value of the argument
    pub expression: EXPR,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<EXPR> std::fmt::Display for UnnamedArgument<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl<T, EXPR> CastInto<UnnamedArgument<T>> for UnnamedArgument<EXPR>
where
    EXPR: CastInto<T>,
{
    fn cast_into(self) -> UnnamedArgument<T> {
        UnnamedArgument {
            expression: self.expression.cast_into(),
            src_ref: self.src_ref,
        }
    }
}

/// *Ordered map* of arguments in a [`Call`].
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "EXPR: Serialize", deserialize = "EXPR: Deserialize<'de>"))]
pub struct ArgumentList<EXPR> {
    /// Source code reference
    pub src_ref: SrcRef,

    /// The unnamed arguments.
    #[serde(skip_serializing_if = "is_default", default)]
    pub unnamed_args: Box<[ir::UnnamedArgument<EXPR>]>,
    /// Named arguments, sorted by name.
    #[serde(skip_serializing_if = "is_default", default)]
    pub named_args: Box<[ir::NamedArgument<EXPR>]>,
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
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound(
    serialize = "EXPR: Serialize, EXPR::Name: Serialize",
    deserialize = "EXPR: Deserialize<'de>, EXPR::Name: Deserialize<'de>"
))]
pub struct Call<EXPR: ir::ExpressionKind> {
    /// Name of the call.
    pub name: EXPR::Name,
    /// Argument list of the call.
    pub argument_list: ir::ArgumentList<EXPR>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<EXPR> std::fmt::Display for Call<EXPR>
where
    EXPR: ir::ExpressionKind + std::fmt::Display,
    EXPR::Name: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}

impl<T: ir::ExpressionKind, EXPR: ir::ExpressionKind> CastInto<Call<T>> for Call<EXPR>
where
    EXPR: crate::CastInto<T>,
    T::Name: From<EXPR::Name>,
{
    fn cast_into(self) -> Call<T> {
        Call {
            name: self.name.into(),
            argument_list: self.argument_list.cast_into(),
            src_ref: self.src_ref,
        }
    }
}
