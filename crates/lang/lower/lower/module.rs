// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{LowerContext, LowerError, ir};

use microcad_lang_parse::ast;

impl ir::ModuleDefinition {
    /// Get inline module
    pub fn from_ast_inline(
        node: &ast::InlineModule,
        context: &mut LowerContext,
    ) -> Result<Self, LowerError> {
        use crate::lower::Lower;
        Ok(Self {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: ir::DocBlock::lower(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|visibility| ir::Visibility::lower(visibility, context))
                .transpose()?
                .unwrap_or_default(),
            id: ir::Identifier::lower(&node.id, context)?,
            body: Some(ir::Body::lower(&node.body, context)?),
        })
    }

    /// Get file module
    pub fn from_ast_file(
        node: &ast::FileModule,
        context: &mut LowerContext,
    ) -> Result<Self, LowerError> {
        use crate::lower::Lower;
        Ok(Self {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: ir::DocBlock::lower(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|visibility| ir::Visibility::lower(visibility, context))
                .transpose()?
                .unwrap_or_default(),
            id: ir::Identifier::lower(&node.id, context)?,
            body: None,
        })
    }
}
