// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};
use microcad_lang_base::Refer;
use microcad_syntax::ast;

impl FromAst for ir::FormatExpression {
    type AstNode = ast::StringExpression;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::FormatExpression::new(
            node.specification
                .is_some()
                .then(|| ir::FormatSpec::from_ast(&node.specification, context))
                .transpose()?,
            ir::Expression::from_ast(&node.expression, context)?,
            context.src_ref(&node.span),
        ))
    }
}

impl FromAst for ir::FormatSpec {
    type AstNode = ast::StringFormatSpecification;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        fn transpose_ref<T: Clone, E: Clone>(opt: &Option<Result<T, E>>) -> Result<Option<&T>, E> {
            match opt.as_ref() {
                None => Ok(None),
                Some(Err(e)) => Err(e.clone()),
                Some(Ok(t)) => Ok(Some(t)),
            }
        }
        Ok(ir::FormatSpec {
            width: transpose_ref(&node.width)
                .map_err(|(e, span)| {
                    LowerError::ParseIntError(Refer::new(e, context.src_ref(&span)))
                })?
                .copied(),
            precision: transpose_ref(&node.precision)
                .map_err(|(e, span)| {
                    LowerError::ParseIntError(Refer::new(e, context.src_ref(&span)))
                })?
                .copied(),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for ir::FormatString {
    type AstNode = ast::FormatString;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        let parts = node
            .parts
            .iter()
            .map(|part| ir::FormatStringInner::from_ast(part, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ir::FormatString(Refer::new(
            parts,
            context.src_ref(&node.span),
        )))
    }
}

impl FromAst for ir::FormatStringInner {
    type AstNode = ast::StringPart;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(match node {
            ast::StringPart::Char(c) => ir::FormatStringInner::String(Refer::new(
                c.character.into(),
                context.src_ref(&c.span),
            )),
            ast::StringPart::Content(s) => ir::FormatStringInner::String(Refer::new(
                s.content.clone(),
                context.src_ref(&s.span),
            )),
            ast::StringPart::Expression(e) => {
                ir::FormatStringInner::FormatExpression(ir::FormatExpression::from_ast(e, context)?)
            }
        })
    }
}
