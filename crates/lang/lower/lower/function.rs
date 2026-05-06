// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};

use microcad_lang_base::Identifier;
use microcad_syntax::ast;

impl FromAst for ir::FunctionDefinition {
    type AstNode = ast::FunctionDefinition;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::FunctionDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: ir::DocBlock::from_ast(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|vis| ir::Visibility::from_ast(vis, context))
                .transpose()?
                .unwrap_or_default(),
            id: Identifier::from_ast(&node.name, context)?,
            body: ir::Body::from_ast(&node.body, context)?,
            signature: ir::FunctionSignature {
                src_ref: context.src_ref(&node.span),
                parameters: ir::ParameterList::from_ast(&node.parameters, context)?,
                return_type: node
                    .return_type
                    .as_ref()
                    .map(|ty| ir::TypeAnnotation::from_ast(ty, context))
                    .transpose()?,
            },
        })
    }
}
