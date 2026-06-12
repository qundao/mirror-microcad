// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

use crate::{Identifiable, ir, is_default};
use microcad_lang_base::{Identifier, SrcRef, SrcReferrer};
use serde::Serialize;

/// NamedArgument in a [`Call`].
#[derive(Debug, PartialEq, Serialize)]
#[serde(bound(serialize = "EXPR: Serialize"))]
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

impl<EXPR> Identifiable for NamedArgument<EXPR> {
    fn id_ref(&self) -> &Identifier {
        &self.id
    }
}

impl<EXPR> SrcReferrer for NamedArgument<EXPR> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

/// Unnamed argument in a [`Call`].
#[derive(Debug, PartialEq, Serialize)]
#[serde(bound(serialize = "EXPR: Serialize"))]
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

/// *Ordered map* of arguments in a [`Call`].
#[derive(Debug, PartialEq, Serialize)]
#[serde(bound(serialize = "EXPR: Serialize"))]
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

/// Call of a *workbench* or *function*.
#[derive(Debug, Serialize)]
#[serde(bound(serialize = "EXPR: Serialize, NAME: Serialize"))]
pub struct Call<EXPR, NAME = ir::QualifiedName> {
    /// Qualified name of the call.
    pub name: NAME,
    /// Argument list of the call.
    pub argument_list: ir::ArgumentList<EXPR>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl<EXPR> std::fmt::Display for Call<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}
