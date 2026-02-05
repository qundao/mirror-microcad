// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;

impl Parse for InitDefinition {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::init_definition);

        Ok(InitDefinition {
            doc: crate::find_rule_opt!(pair, doc_block)?,
            parameters: crate::find_rule!(pair, parameter_list)?,
            body: crate::find_rule!(pair, body)?,
            src_ref: pair.into(),
        })
    }
}

impl FromAst for InitDefinition {
    type AstNode = ast::InitDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(InitDefinition {
            doc: node
                .doc
                .as_ref()
                .map(|doc| DocBlock::from_ast(doc, context))
                .transpose()?,
            parameters: ParameterList::from_ast(&node.arguments, context)?,
            body: Body::from_ast(&node.body, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
