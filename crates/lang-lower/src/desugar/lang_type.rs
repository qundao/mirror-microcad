// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Desugar, LowerContext, LowerResult, ir};
use microcad_lang_parse::ast;
use microcad_lang_types::ty;

impl Desugar<ast::TupleType> for ir::TupleType {
    fn desugar(node: &ast::TupleType, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            named: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.as_ref().map(|name| (name, value)))
                .map(|(name, value)| -> LowerResult<(_, _)> {
                    let name = ir::Identifier::desugar(name, context)?;
                    let value = ty::Type::desugar(value, context)?;
                    Ok((name, value))
                })
                .collect::<Result<Vec<(_, _)>, _>>()?,
            positional: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.is_none().then_some(value))
                .map(|value| ty::Type::desugar(value, context))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
