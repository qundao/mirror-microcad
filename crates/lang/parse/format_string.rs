// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

impl Parse for FormatExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self::new(
            crate::find_rule_opt!(pair, format_spec)?,
            crate::find_rule!(pair, expression)?,
            pair.into(),
        ))
    }
}

impl FromAst for FormatExpression {
    type AstNode = ast::StringExpression;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(FormatExpression::new(
            node.specification
                .is_some()
                .then(|| FormatSpec::from_ast(&node.specification, context))
                .transpose()?,
            Expression::from_ast(&node.expression, context)?,
            context.src_ref(&node.span),
        ))
    }
}

impl Parse for FormatSpec {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut opt = FormatSpec::default();

        for pair in pair.inner() {
            match pair.as_span().as_str()[1..].parse() {
                Ok(parsed) => match pair.as_rule() {
                    Rule::format_spec_precision => opt.precision = Some(parsed),
                    Rule::format_spec_width => opt.width = Some(parsed),
                    _ => unreachable!(),
                },
                Err(err) => return Err(ParseError::ParseIntError(Refer::new(err, pair.into()))),
            }
        }

        opt.src_ref = pair.into();

        Ok(opt)
    }
}

impl FromAst for FormatSpec {
    type AstNode = ast::StringFormatSpecification;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        fn transpose_ref<T: Clone, E: Clone>(opt: &Option<Result<T, E>>) -> Result<Option<&T>, E> {
            match opt.as_ref() {
                None => Ok(None),
                Some(Err(e)) => Err(e.clone()),
                Some(Ok(t)) => Ok(Some(t)),
            }
        }
        Ok(FormatSpec {
            width: transpose_ref(&node.width)
                .map_err(|(e, span)| {
                    ParseError::ParseIntError(Refer::new(e, context.src_ref(&span)))
                })?
                .copied(),
            precision: transpose_ref(&node.precision)
                .map_err(|(e, span)| {
                    ParseError::ParseIntError(Refer::new(e, context.src_ref(&span)))
                })?
                .copied(),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Parse for FormatString {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut fs = Self::default();
        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::string_literal_inner => {
                    fs.push_string(pair.as_span().as_str().to_string(), pair.into())
                }
                Rule::format_expression => fs.push_format_expr(FormatExpression::parse(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(fs)
    }
}

impl FromAst for FormatString {
    type AstNode = ast::FormatString;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let parts = node
            .parts
            .iter()
            .map(|part| FormatStringInner::from_ast(part, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FormatString(Refer::new(parts, context.src_ref(&node.span))))
    }
}

impl std::str::FromStr for FormatString {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Parser::parse_rule::<Self>(Rule::format_string, s, 0)
    }
}

impl FromAst for FormatStringInner {
    type AstNode = ast::StringPart;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(match node {
            ast::StringPart::Char(c) => {
                FormatStringInner::String(Refer::new(c.character.into(), context.src_ref(&c.span)))
            }
            ast::StringPart::Content(s) => {
                FormatStringInner::String(Refer::new(s.content.clone(), context.src_ref(&s.span)))
            }
            ast::StringPart::Expression(e) => {
                FormatStringInner::FormatExpression(FormatExpression::from_ast(e, context)?)
            }
        })
    }
}
