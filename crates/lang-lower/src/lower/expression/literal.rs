// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, LowerResult, ir};

use microcad_lang_base::{Refer, SpanToSrcRef};
use microcad_lang_parse::ast;
use microcad_lang_types::{Integer, Quantity, Scalar};

impl Lower<ast::Literal> for ir::Literal {
    fn lower(node: &ast::Literal, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match &node.literal {
            ast::LiteralKind::Bool(lit) => ir::Literal(Refer::new(
                lit.value.into(),
                context.span_to_src_ref(&lit.span),
            )),
            ast::LiteralKind::Integer(lit) => ir::Literal(Refer::new(
                Integer::from_str(lit.value.as_str()).expect(
                    "No error expected, this string already has been checked in the parse stage.",
                ).into(),
                context.span_to_src_ref(&lit.span),
            )),
            ast::LiteralKind::Float(lit) => ir::Literal(Refer::new(
                Scalar::from_str(lit.value.as_str()).expect(
                    "No error expected, this string already has been checked in the parse stage.",
                ).into(),
                context.span_to_src_ref(&lit.span),
            )),
            ast::LiteralKind::Quantity(lit) => {
                let unit = ir::Unit::lower(&lit.unit, context)?;
                ir::Literal(Refer::new(
                    Quantity {
                        value: unit.normalize(Scalar::from_str(lit.value.as_str()).expect("No error expected, this string already has been checked in the parse stage.")),
                        quantity_type: unit.quantity_type(),
                        unit,
                    }
                    .into(),
                    context.span_to_src_ref(&lit.span),
                ))
            }
            ast::LiteralKind::String(lit) => ir::Literal(Refer::new(
                lit.content.clone().into(),
                context.span_to_src_ref(&lit.span),
            )),
            ast::LiteralKind::Error(e) => {
                return Err(LowerError::InvalidLiteral {
                    error: e.kind.clone(),
                    src_ref: context.span_to_src_ref(&e.span),
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
                context.span_to_src_ref(&node.span),
            ))
        })
    }
}

impl Lower<Option<ast::Unit>> for ir::Unit {
    fn lower(node: &Option<ast::Unit>, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(node
            .as_ref()
            .map(|unit| Self::lower(unit, context))
            .transpose()?
            .unwrap_or_default())
    }
}
