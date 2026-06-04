// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

mod argument;
mod argument_list;
mod method_call;

pub use argument::*;
pub use argument_list::*;
pub use method_call::*;
use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::ir;

/// Call of a *workbench* or *function*.
#[derive(Clone, Debug, Default, SrcReferrer)]
pub struct Call<EXPR = ir::Expression> {
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
