// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    lower::{Lower, LowerContext, LowerError, ir},
    value::Quantity,
};

use microcad_lang_base::{Refer, SrcRef};
use microcad_lang_parse::ast;

impl Lower for ir::Literal {
    type AstNode = ast::Literal;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
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
                    Quantity {
                        value: unit.normalize(lit.value),
                        quantity_type: unit.quantity_type(),
                        unit,
                    }
                    .into(),
                    context.src_ref(&lit.span),
                ))
            }
            ast::LiteralKind::String(_) => {
                unreachable!("string literal are handled else were");
            }
            ast::LiteralKind::Error(e) => {
                return Err(LowerError::InvalidLiteral {
                    error: e.kind.clone(),
                    src_ref: context.src_ref(&e.span),
                });
            }
        })
    }
}

impl Lower for ir::NumberLiteral {
    type AstNode = ast::QuantityLiteral;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::NumberLiteral(
            node.value,
            ir::Unit::lower(&node.unit, context)?,
            context.src_ref(&node.span),
        ))
    }
}

impl std::str::FromStr for ir::NumberLiteral {
    type Err = LowerError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let num_bytes = s
            .bytes()
            .take_while(|&c| c == b'-' || c.is_ascii_digit() || c == b'.')
            .count();
        let value = s[0..num_bytes]
            .parse()
            .map_err(|e| LowerError::InvalidLiteral {
                error: ast::LiteralErrorKind::Float(e),
                src_ref: SrcRef::default(),
            })?;
        let unit = s.get(num_bytes..).map(ir::Unit::from_str).transpose()?;
        Ok(ir::NumberLiteral(
            value,
            unit.unwrap_or_default(),
            SrcRef::default(),
        ))
    }
}

impl Lower for ir::Unit {
    type AstNode = ast::Unit;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        use std::str::FromStr;
        ir::Unit::from_str(node.name.as_str()).map_err(|_| {
            LowerError::UnknownUnit(Refer::new(
                node.name.to_string(),
                context.src_ref(&node.span),
            ))
        })
    }
}

impl std::str::FromStr for ir::Unit {
    type Err = LowerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Scalars
            "" => Ok(Self::None),
            "%" => Ok(Self::Percent),

            // Lengths
            "m" => Ok(Self::Meter),
            "cm" => Ok(Self::Centimeter),
            "mm" => Ok(Self::Millimeter),
            "µm" => Ok(Self::Micrometer),
            "in" => Ok(Self::Inch),
            "\"" => Ok(Self::Inch),
            "ft" => Ok(Self::Foot),
            "\'" => Ok(Self::Foot),
            "yd" => Ok(Self::Yard),

            // Angles
            "deg" => Ok(Self::Deg),
            "°" => Ok(Self::DegS),
            "grad" => Ok(Self::Grad),
            "turns" => Ok(Self::Turns),
            "rad" => Ok(Self::Rad),

            // Weights
            "g" => Ok(Self::Gram),
            "kg" => Ok(Self::Kilogram),
            "lb" => Ok(Self::Pound),
            "oz" => Ok(Self::Ounce),

            // Areas
            "m²" | "m2" => Ok(Self::Meter2),
            "cm²" | "cm2" => Ok(Self::Centimeter2),
            "mm²" | "mm2" => Ok(Self::Millimeter2),
            "µm²" | "µm2" => Ok(Self::Micrometer2),
            "in²" | "in2" => Ok(Self::Inch2),
            "ft²" | "ft2" => Ok(Self::Foot2),
            "yd²" | "yd2" => Ok(Self::Yard2),

            // Volumes
            "m³" | "m3" => Ok(Self::Meter3),
            "cm³" | "cm3" => Ok(Self::Centimeter3),
            "mm³" | "mm3" => Ok(Self::Millimeter3),
            "µm³" | "µm3" => Ok(Self::Micrometer3),
            "in³" | "in3" => Ok(Self::Inch3),
            "ft³" | "ft3" => Ok(Self::Foot3),
            "yd³" | "yd3" => Ok(Self::Yard3),
            "ml" => Ok(Self::Milliliter),
            "cl" => Ok(Self::Centiliter),
            "l" => Ok(Self::Liter),
            "µl" => Ok(Self::Microliter),

            "g/mm³" => Ok(Self::GramPerMillimeter3),
            "g/m³" => Ok(Self::GramPerMeter3),

            // Unknown
            _ => Err(LowerError::UnknownUnit(Refer::new(
                s.into(),
                SrcRef::default(),
            ))),
        }
    }
}
