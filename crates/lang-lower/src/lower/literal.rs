// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, LowerResult, ir};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;
use microcad_lang_types::value;

impl Lower<ast::Literal> for ir::Literal {
    fn lower(node: &ast::Literal, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match &node.literal {
            ast::LiteralKind::Bool(lit) => {
                ir::Literal(Refer::new(lit.value.into(), context.src_ref(&lit.span)))
            }
            ast::LiteralKind::Integer(lit) => {
                ir::Literal(Refer::new(lit.value.into(), context.src_ref(&lit.span)))
            }
            ast::LiteralKind::Float(lit) => {
                ir::Literal(Refer::new(lit.value.into(), context.src_ref(&lit.span)))
            }
            ast::LiteralKind::Quantity(lit) => {
                let unit = ir::Unit::lower(&lit.unit, context)?;
                ir::Literal(Refer::new(
                    value::Quantity {
                        value: unit.normalize(lit.value),
                        quantity_type: unit.quantity_type(),
                        unit,
                    }
                    .into(),
                    context.src_ref(&lit.span),
                ))
            }
            ast::LiteralKind::String(lit) => ir::Literal(Refer::new(
                lit.content.clone().into(),
                context.src_ref(&lit.span),
            )),
            ast::LiteralKind::Error(e) => {
                return Err(LowerError::InvalidLiteral {
                    error: e.kind.clone(),
                    src_ref: context.src_ref(&e.span),
                });
            }
        })
    }
}

impl Lower<ast::Unit> for ir::Unit {
    fn lower(node: &ast::Unit, context: &mut LowerContext) -> LowerResult<Self> {
        use std::str::FromStr;
        ir::Unit::from_str(node.name.as_str()).map_err(|_| {
            LowerError::UnknownUnit(Refer::new(
                node.name.to_string(),
                context.src_ref(&node.span),
            ))
        })
    }
}
