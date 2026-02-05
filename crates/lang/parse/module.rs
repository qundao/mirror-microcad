// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};
use microcad_syntax::ast;

impl Parse for Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Rc::new(ModuleDefinition {
            doc: crate::find_rule_opt!(pair, doc_block)?,
            visibility: crate::find_rule!(pair, visibility)?,
            id: crate::find_rule!(pair, identifier)?,
            body: crate::find_rule_opt!(pair, body)?,
        }))
    }
}

impl FromAst for ModuleDefinition {
    type AstNode = ast::ModuleDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(ModuleDefinition {
            doc: None, // todo
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
