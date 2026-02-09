// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*, syntax::*};
use microcad_syntax::ast;

impl FromAst for Identifier {
    type AstNode = ast::Identifier;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Self(Refer::new(node.name.clone(), context.src_ref(&node.span))))
    }
}

impl FromAst for QualifiedName {
    type AstNode = ast::QualifiedName;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let parts = node.parts.iter()
            .map(|ident| Identifier::from_ast(ident, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(parts, context.src_ref(&node.span)))
    }
}
