// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

impl FromAst for FunctionDefinition {
    type AstNode = ast::FunctionDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(FunctionDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: node
                .doc
                .as_ref()
                .map(|doc| DocBlock::from_ast(doc, context))
                .transpose()?,
            visibility: node
                .visibility
                .as_ref()
                .map(|vis| Visibility::from_ast(vis, context))
                .transpose()?
                .unwrap_or_default(),
            id: Identifier::from_ast(&node.name, context)?,
            body: Body::from_ast(&node.body, context)?,
            signature: FunctionSignature {
                src_ref: context.src_ref(&node.span),
                parameters: ParameterList::from_ast(&node.arguments, context)?,
                return_type: node
                    .return_type
                    .as_ref()
                    .map(|ty| TypeAnnotation::from_ast(ty, context))
                    .transpose()?,
            },
        })
    }
}
