// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};
use microcad_syntax::ast;

impl FromAst for ir::InitDefinition {
    type AstNode = ast::InitDefinition;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::InitDefinition {
            doc: ir::DocBlock::from_ast(&node.doc, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            parameters: ir::ParameterList::from_ast(&node.parameters, context)?,
            body: ir::Body::from_ast(&node.body, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
