// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

impl FromAst for ModuleDefinition {
    type AstNode = ast::ModuleDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(ModuleDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: node.doc.as_ref().map(|doc| DocBlock::from_ast(doc, context)).transpose()?,
            visibility: node
                .visibility
                .as_ref()
                .map(|visibility| Visibility::from_ast(visibility, context))
                .transpose()?
                .unwrap_or_default(),
            id: Identifier::from_ast(&node.name, context)?,
            body: node
                .body
                .as_ref()
                .map(|body| Body::from_ast(body, context))
                .transpose()?,
        })
    }
}
