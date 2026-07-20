// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, LowerResult, ir};
use microcad_lang_base::{Refer, SpanToSrcRef};
use microcad_lang_parse::ast;
use serde::Serialize;

impl<NAME> Lower<ast::StringExpression> for ir::FormatExpression<NAME>
where
    NAME: Serialize + Lower<ast::SymbolPath>,
{
    fn lower(node: &ast::StringExpression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self::new(
            node.specification
                .is_some()
                .then(|| ir::FormatSpec::lower(&node.specification, context))
                .transpose()?,
            ir::ConstantExpression::lower(&node.expr, context)?,
            context.span_to_src_ref(&node.span),
        ))
    }
}

impl Lower<ast::StringFormatSpecification> for ir::FormatSpec {
    fn lower(
        node: &ast::StringFormatSpecification,
        context: &mut LowerContext,
    ) -> LowerResult<Self> {
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
                    LowerError::ParseIntError(Refer::new(e, context.span_to_src_ref(&span)))
                })?
                .copied(),
            precision: transpose_ref(&node.precision)
                .map_err(|(e, span)| {
                    LowerError::ParseIntError(Refer::new(e, context.span_to_src_ref(&span)))
                })?
                .copied(),
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl<NAME> Lower<ast::FormatString> for ir::FormatString<NAME>
where
    NAME: Serialize + Lower<ast::SymbolPath>,
{
    fn lower(node: &ast::FormatString, context: &mut LowerContext) -> LowerResult<Self> {
        let parts = node
            .parts
            .iter()
            .map(|part| ir::FormatStringInner::lower(part, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ir::FormatString(Refer::new(
            parts,
            context.span_to_src_ref(&node.span),
        )))
    }
}

impl<NAME> Lower<ast::StringPart> for ir::FormatStringInner<NAME>
where
    NAME: Serialize + Lower<ast::SymbolPath>,
{
    fn lower(node: &ast::StringPart, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::StringPart::Char(c) => ir::FormatStringInner::String(Refer::new(
                c.character.into(),
                context.span_to_src_ref(&c.span),
            )),
            ast::StringPart::Content(s) => ir::FormatStringInner::String(Refer::new(
                s.content.clone(),
                context.span_to_src_ref(&s.span),
            )),
            ast::StringPart::Expression(e) => ir::FormatStringInner::FormatExpression(Box::new(
                ir::FormatExpression::lower(e, context)?,
            )),
        })
    }
}

impl<NAME> Lower<ast::StringLiteral> for ir::FormatString<NAME>
where
    NAME: Serialize + Lower<ast::SymbolPath>,
{
    fn lower(node: &ast::StringLiteral, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(Refer::new(
            vec![ir::FormatStringInner::String(Refer::new(
                node.content.clone(),
                context.span_to_src_ref(&node.span),
            ))],
            context.span_to_src_ref(&node.span),
        )))
    }
}
