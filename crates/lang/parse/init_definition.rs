// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

impl FromAst for InitDefinition {
    type AstNode = ast::InitDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(InitDefinition {
            doc: DocBlock::from_ast(&node.doc, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            parameters: ParameterList::from_ast(&node.parameters, context)?,
            body: Body::from_ast(&node.body, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
