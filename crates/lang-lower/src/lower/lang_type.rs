// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir};
use microcad_lang_parse::ast;
use microcad_lang_types::ty;

impl Lower<ast::TupleType> for ir::TupleType {
    fn lower(node: &ast::TupleType, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            named: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.as_ref().map(|name| (name, value)))
                .map(|(name, value)| -> LowerResult<(_, _)> {
                    let name = ir::Identifier::lower(name, context)?;
                    let value = ty::Type::lower(value, context)?;
                    Ok((name, value))
                })
                .collect::<Result<microcad_lang_base::HashMap<_, _>, _>>()?,
            unnamed: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.is_none().then_some(value))
                .map(|value| ty::Type::lower(value, context))
                .collect::<Result<microcad_lang_base::HashSet<_>, _>>()?,
        })
    }
}
