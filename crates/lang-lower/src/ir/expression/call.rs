// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

use crate::{LowerContext, LowerResult, ir};
use derive_more::{Deref, DerefMut};
use microcad_lang_base::{Identifier, OrdMap, OrdMapValue, PushDiag, Refer, SrcRef};

/// Argument in a [`Call`].
#[derive(Debug, PartialEq)]
pub struct Argument<EXPR> {
    /// Name of the argument
    pub id: Option<Identifier>,
    /// Value of the argument
    pub expression: EXPR,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<EXPR> OrdMapValue<Identifier> for Argument<EXPR> {
    fn key(&self) -> Option<Identifier> {
        self.id.clone()
    }
}

impl<EXPR> std::fmt::Display for Argument<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.id {
            Some(ref id) => write!(f, "{id} = {}", self.expression),
            None => write!(f, "{}", self.expression),
        }
    }
}

/// *Ordered map* of arguments in a [`Call`].
#[derive(Debug, Deref, DerefMut, PartialEq)]
pub struct ArgumentList<EXPR>(pub Refer<OrdMap<Identifier, ir::Argument<EXPR>>>);

impl<EXPR> ArgumentList<EXPR> {
    pub(crate) fn new() -> Self {
        Self(Refer::none(microcad_lang_base::OrdMap::<
            ir::Identifier,
            ir::Argument<EXPR>,
        >::default()))
    }

    pub fn try_push(
        &mut self,
        arg: ir::Argument<EXPR>,
        context: &mut LowerContext,
    ) -> LowerResult<()> {
        let src_ref = arg.src_ref.clone();
        match self.0.value.push(arg) {
            Some(_) => {
                context
                    .diagnostics
                    .error(&src_ref, miette::miette!("Duplicated argument"))
                    .ok(); // TODO Better error handling
            }
            None => {}
        }

        Ok(())
    }
}

impl<EXPR> std::fmt::Display for ArgumentList<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let mut v = self
                .0
                .value
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>();
            v.sort();
            v.join(", ")
        })
    }
}

impl<EXPR> Default for ArgumentList<EXPR> {
    fn default() -> Self {
        Self(Refer::none(OrdMap::default()))
    }
}

impl<EXPR> std::ops::Index<&Identifier> for ArgumentList<EXPR> {
    type Output = ir::Argument<EXPR>;

    fn index(&self, name: &Identifier) -> &Self::Output {
        self.0.get(name).expect("key not found")
    }
}

impl<EXPR> std::ops::Index<usize> for ArgumentList<EXPR> {
    type Output = ir::Argument<EXPR>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0.value[idx]
    }
}

/// Call of a *workbench* or *function*.
#[derive(Debug, Default)]
pub struct Call<EXPR> {
    /// Qualified name of the call.
    pub name: ir::QualifiedName,
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
