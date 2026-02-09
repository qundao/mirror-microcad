// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

impl FromAst for Body {
    type AstNode = ast::StatementList;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Body {
            statements: StatementList::from_ast(node, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}