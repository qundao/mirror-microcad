// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};

use microcad_lang_base::Refer;
use microcad_syntax::ast;

impl FromAst for ir::DocBlock {
    type AstNode = ast::DocBlock;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::DocBlock(Refer::new(
            node.lines.clone(),
            context.src_ref(&node.span),
        )))
    }
}

impl FromAst for ir::InnerDocComment {
    type AstNode = ast::InnerDocComment;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(Self(Refer::new(
            node.line.clone(),
            context.src_ref(&node.span),
        )))
    }
}
