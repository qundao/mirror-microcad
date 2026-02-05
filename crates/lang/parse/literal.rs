// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_core::Integer;
use microcad_syntax::ast;
use std::str::FromStr;

impl Parse for Refer<Integer> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.as_str().parse::<i64>() {
            Ok(value) => Ok(Refer::new(value, pair.into())),
            Err(err) => Err(ParseError::ParseIntError(Refer::new(err, pair.into()))),
        }
    }
}

impl Parse for Literal {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::literal);

        let inner = pair.inner().next().expect(INTERNAL_PARSE_ERROR);

        let s = match inner.as_rule() {
            Rule::number_literal => Literal::Number(NumberLiteral::parse(inner)?),
            Rule::integer_literal => Literal::Integer(Refer::<Integer>::parse(inner)?),
            Rule::bool_literal => match inner.as_str() {
                "true" => Literal::Bool(Refer::new(true, pair.into())),
                "false" => Literal::Bool(Refer::new(false, pair.into())),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        Ok(s)
    }
}

impl FromAst for Literal {
    type AstNode = ast::Literal;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(match &node.literal {
            ast::LiteralKind::Bool(lit) => {
                Literal::Bool(Refer::new(lit.value, context.src_ref(&lit.span)))
            }
            ast::LiteralKind::Integer(lit) => {
                Literal::Integer(Refer::new(lit.value, context.src_ref(&lit.span)))
            }
            ast::LiteralKind::Quantity(lit) => {
                Literal::Number(NumberLiteral::from_ast(lit, context)?)
            }
            ast::LiteralKind::Float(lit) => {
                Literal::Number(NumberLiteral(lit.value, Unit::None, context.src_ref(&lit.span)))
            }
            ast::LiteralKind::String(_) => {
                unreachable!("string literal are handled else were");
            }
            ast::LiteralKind::Error(e) => return Err(ParseError::InvalidLiteral {
                error: e.kind.clone(),
                src_ref: context.src_ref(&e.span),
            }),
        })
    }
}

impl std::str::FromStr for Literal {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Parser::parse_rule::<Self>(Rule::literal, s, 0)
    }
}

impl FromAst for NumberLiteral {
    type AstNode = ast::QuantityLiteral;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(NumberLiteral(
            node.value,
            Unit::from_ast(&node.ty, context)?,
            context.src_ref(&node.span),
        ))
    }
}

impl Parse for NumberLiteral {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::number_literal);

        let mut inner = pair.inner();
        let number_token = inner.next().expect("Expected number token");

        assert!(
            number_token.as_rule() == Rule::number
                || number_token.as_rule() == Rule::integer_literal
        );

        let value = match number_token.as_str().parse::<f64>() {
            Ok(value) => value,
            Err(err) => return Err(ParseError::ParseFloatError(Refer::new(err, pair.src_ref()))),
        };

        let mut unit = Unit::None;

        if let Some(unit_token) = inner.next() {
            unit = Unit::parse(unit_token)?;
        }
        Ok(NumberLiteral(value, unit, pair.clone().into()))
    }
}

impl std::str::FromStr for NumberLiteral {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Parser::parse_rule(Rule::number_literal, s, 0)
    }
}

impl Parse for Unit {
    fn parse(pair: Pair) -> ParseResult<Self> {
        use std::str::FromStr;
        match Unit::from_str(pair.as_str()) {
            Ok(unit) => Ok(unit),
            Err(()) => Err(ParseError::UnknownUnit(Refer::new(
                pair.as_str().to_string(),
                pair.into(),
            ))),
        }
    }
}

impl FromAst for Unit {
    type AstNode = ast::SingleType;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Unit::from_str(node.name.as_str()).map_err(|_| {
            ParseError::UnknownUnit(Refer::new(
                node.name.to_string(),
                context.src_ref(&node.span),
            ))
        })
    }
}

impl std::str::FromStr for Unit {
    type Err = ();
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
            _ => Err(()),
        }
    }
}
