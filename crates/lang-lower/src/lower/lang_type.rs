// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    lower::{Lower, LowerContext, LowerError, ir},
    ty::*,
};
use microcad_lang_parse::ast;

impl Lower for ir::TupleType {
    type AstNode = ast::TupleType;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(TupleType {
            named: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.as_ref().map(|name| (name, value)))
                .map(|(name, value)| {
                    let name = ir::Identifier::lower(name, context)?;
                    let value = Type::lower(value, context)?;
                    Ok((name, value))
                })
                .collect::<Result<microcad_core::hash::HashMap<_, _>, _>>()?,
            unnamed: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.is_none().then_some(value))
                .map(|value| Type::lower(value, context))
                .collect::<Result<microcad_core::hash::HashSet<_>, _>>()?,
        })
    }
}
