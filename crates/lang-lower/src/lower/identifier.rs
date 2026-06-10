// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, ir};
use microcad_lang_base::Refer;
use microcad_lang_parse::ast;

impl Lower for ir::Identifier {
    type AstNode = ast::Identifier;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(Refer::new(
            node.name.clone(),
            context.src_ref(&node.span),
        )))
    }
}
