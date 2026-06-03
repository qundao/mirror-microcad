// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple expression.

use crate::lower::ir;

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

/// Tuple expression, e.g. `(x=1+2,4,z=9)`.
#[derive(Clone, Debug, Default, PartialEq, SrcReferrer)]
pub struct TupleExpression<EXPR = ir::Expression> {
    /// List of tuple members.
    pub args: ir::ArgumentList<EXPR>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<EXPR> std::fmt::Display for TupleExpression<EXPR>
where
    EXPR: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.args
                .iter()
                .map(|arg| if let Some(name) = &arg.id {
                    format!("{} = {}", &name, arg.expression)
                } else {
                    arg.to_string()
                })
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        Ok(())
    }
}
