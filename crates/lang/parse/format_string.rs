// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

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
