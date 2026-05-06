// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    lower::{FromAst, LowerContext, LowerError, ir},
    ty::*,
};
use microcad_syntax::ast;

impl FromAst for ir::TupleType {
    type AstNode = ast::TupleType;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(TupleType {
            named: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.as_ref().map(|name| (name, value)))
                .map(|(name, value)| {
                    let name = ir::Identifier::from_ast(name, context)?;
                    let value = Type::from_ast(value, context)?;
                    Ok((name, value))
                })
                .collect::<Result<microcad_core::hash::HashMap<_, _>, _>>()?,
            unnamed: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.is_none().then_some(value))
                .map(|value| Type::from_ast(value, context))
                .collect::<Result<microcad_core::hash::HashSet<_>, _>>()?,
        })
    }
}
