// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};

use microcad_lang_base::Identifier;
use microcad_lang_parse::ast;

impl Lower for ir::FunctionDefinition {
    type AstNode = ast::FunctionDefinition;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::FunctionDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: ir::DocBlock::lower(&node.doc, context)?,
            visibility: node
                .vis
                .as_ref()
                .map(|vis| ir::Visibility::lower(vis, context))
                .transpose()?
                .unwrap_or_default(),
            id: Identifier::lower(&node.id, context)?,
            body: ir::Body::lower(&node.body, context)?,
            signature: ir::FunctionSignature {
                src_ref: context.src_ref(&node.span),
                parameters: ir::ParameterList::lower(&node.parameters, context)?,
                return_type: node
                    .return_type
                    .as_ref()
                    .map(|ty| ir::TypeAnnotation::lower(ty, context))
                    .transpose()?,
            },
        })
    }
}
