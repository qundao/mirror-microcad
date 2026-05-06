// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{LowerContext, LowerError, ir};

use microcad_syntax::ast;

impl ir::ModuleDefinition {
    /// Get inline module
    pub fn from_ast_inline(
        node: &ast::InlineModule,
        context: &LowerContext,
    ) -> Result<Self, LowerError> {
        use crate::lower::FromAst;
        Ok(Self {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: ir::DocBlock::from_ast(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|visibility| ir::Visibility::from_ast(visibility, context))
                .transpose()?
                .unwrap_or_default(),
            id: ir::Identifier::from_ast(&node.name, context)?,
            body: Some(ir::Body::from_ast(&node.body, context)?),
        })
    }

    /// Get file module
    pub fn from_ast_file(
        node: &ast::FileModule,
        context: &LowerContext,
    ) -> Result<Self, LowerError> {
        use crate::lower::FromAst;
        Ok(Self {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: ir::DocBlock::from_ast(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|visibility| ir::Visibility::from_ast(visibility, context))
                .transpose()?
                .unwrap_or_default(),
            id: ir::Identifier::from_ast(&node.name, context)?,
            body: None,
        })
    }
}
