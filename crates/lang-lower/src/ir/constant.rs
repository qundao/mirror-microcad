// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ir;

/// A constant definition: `const FOO: Length = 32mm`.
#[skip_serializing_none]
#[derive(Debug, Clone, SrcReferrer, Hash, PartialEq, Serialize, Deserialize)]
pub struct Constant<Path: ir::PathSpec = ir::Path> {
    pub src_ref: SrcRef,
    pub attr: ir::OuterAttributes<Path>,
    pub visibility: ir::Visibility,
    #[serde(skip_serializing_if = "SrcRef::is_none", default)]
    pub keyword_src_ref: SrcRef,
    pub id: ir::Identifier,
    pub ty: ir::Type,
    pub expr: ir::ConstantExpression<Path>,
}

impl<Path: ir::PathSpec> std::fmt::Display for Constant<Path>
where
    Path: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{vis}const {id}: {ty} = {expr}",
            vis = self.visibility,
            id = self.id,
            ty = self.ty,
            expr = self.expr
        )
    }
}
