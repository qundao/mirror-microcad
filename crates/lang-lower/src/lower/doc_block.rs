// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;

impl Lower for ir::DocBlock {
    type AstNode = ast::DocBlock;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::DocBlock(Refer::new(
            node.lines.clone(),
            context.src_ref(&node.span),
        )))
    }
}

impl Lower for ir::InnerDocComment {
    type AstNode = ast::InnerDocComment;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(Self(Refer::new(
            node.line.clone(),
            context.src_ref(&node.span),
        )))
    }
}
