// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};
use microcad_syntax::ast;

impl Lower for ir::Body {
    type AstNode = ast::Body;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Body {
            statements: ir::StatementList::lower(&node.statements, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
