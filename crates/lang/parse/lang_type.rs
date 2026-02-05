// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::{HashMap, HashSet};
use crate::{parse::*, parser::*, syntax::*, ty::*};
use microcad_syntax::ast;

impl FromAst for TupleType {
    type AstNode = ast::TupleType;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(TupleType {
            named: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.as_ref().map(|name| (name, value)))
                .map(|(name, value)| {
                    let name = Identifier::from_ast(name, context)?;
                    let value = Type::from_ast(value, context)?;
                    Ok((name, value))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
            unnamed: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.is_none().then_some(value))
                .map(|value| Type::from_ast(value, context))
                .collect::<Result<HashSet<_>, _>>()?,
        })
    }
}