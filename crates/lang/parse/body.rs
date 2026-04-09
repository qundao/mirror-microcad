// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

impl FromAst for Body {
    type AstNode = ast::Body;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Body {
            statements: StatementList::from_ast(&node.statements, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
