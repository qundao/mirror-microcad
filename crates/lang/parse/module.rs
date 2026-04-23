// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

impl ModuleDefinition {
    /// Get inline module
    pub fn from_ast_inline(
        node: &ast::InlineModule,
        context: &ParseContext,
    ) -> Result<Self, ParseError> {
        Ok(ModuleDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: DocBlock::from_ast(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|visibility| Visibility::from_ast(visibility, context))
                .transpose()?
                .unwrap_or_default(),
            id: Identifier::from_ast(&node.name, context)?,
            body: Some(Body::from_ast(&node.body, context)?),
        })
    }

    /// Get file module
    pub fn from_ast_file(
        node: &ast::FileModule,
        context: &ParseContext,
    ) -> Result<Self, ParseError> {
        Ok(ModuleDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: DocBlock::from_ast(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|visibility| Visibility::from_ast(visibility, context))
                .transpose()?
                .unwrap_or_default(),
            id: Identifier::from_ast(&node.name, context)?,
            body: None,
        })
    }
}
