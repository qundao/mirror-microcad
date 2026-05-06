// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};
use microcad_lang_base::Refer;
use microcad_syntax::ast;

impl Lower for ir::Identifier {
    type AstNode = ast::Identifier;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(Self(Refer::new(
            node.name.clone(),
            context.src_ref(&node.span),
        )))
    }
}

impl Lower for ir::QualifiedName {
    type AstNode = ast::QualifiedName;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        let parts = node
            .parts
            .iter()
            .map(|ident| ir::Identifier::lower(ident, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(parts, context.src_ref(&node.span)))
    }
}
