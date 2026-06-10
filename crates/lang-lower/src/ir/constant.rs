// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::ir;

/// A constant definition: `const FOO: Length = 32mm`.
#[derive(Debug, SrcReferrer)]
pub struct Constant {
    pub src_ref: SrcRef,
    pub attr: ir::Attributes,
    pub visibility: ir::Visibility,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
    pub id: ir::Identifier,
    pub ty: Option<ir::TypeAnnotation>,
    pub expr: ir::ConstantExpression,
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.ty {
            Some(ty) => write!(
                f,
                "{vis}const {id}: {ty} = {expr}",
                vis = self.visibility,
                id = self.id,
                expr = self.expr
            ),
            None => write!(
                f,
                "{vis}const {id} = {expr}",
                vis = self.visibility,
                id = self.id,
                expr = self.expr
            ),
        }
    }
}

/// A list of constants
#[derive(Debug)]
pub struct Constants(pub Box<[Constant]>);
