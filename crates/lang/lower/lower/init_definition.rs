// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};
use microcad_syntax::ast;

impl Lower for ir::InitDefinition {
    type AstNode = ast::InitDefinition;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::InitDefinition {
            doc: ir::DocBlock::lower(&node.doc, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            body: ir::Body::lower(&node.body, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
