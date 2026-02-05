// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};
use microcad_syntax::ast;

impl Parse for Rc<FunctionDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_definition);

        Ok(Rc::new(FunctionDefinition {
            doc: crate::find_rule_opt!(pair, doc_block)?,
            visibility: crate::find_rule!(pair, visibility)?,
            id: crate::find_rule!(pair, identifier)?,
            signature: crate::find_rule_exact!(pair, function_signature)?,
            body: crate::find_rule!(pair, body)?,
        }))
    }
}

impl FromAst for FunctionDefinition {
    type AstNode = ast::FunctionDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(FunctionDefinition {
            doc: node
                .doc
                .as_ref()
                .map(|doc| DocBlock::from_ast(doc, context))
                .transpose()?,
            visibility: node
                .visibility
                .as_ref()
                .map(|vis| Visibility::from_ast(vis, context))
                .transpose()?
                .unwrap_or_default(),
            id: Identifier::from_ast(&node.name, context)?,
            body: Body::from_ast(&node.body, context)?,
            signature: FunctionSignature {
                src_ref: context.src_ref(&node.span),
                parameters: ParameterList::from_ast(&node.arguments, context)?,
                return_type: node
                    .return_type
                    .as_ref()
                    .map(|ty| TypeAnnotation::from_ast(ty, context))
                    .transpose()?,
            },
        })
    }
}

impl Parse for FunctionSignature {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut parameters = ParameterList::default();
        let mut return_type = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?;
                }
                Rule::r#type => return_type = Some(TypeAnnotation::parse(pair)?),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        Ok(Self {
            parameters,
            return_type,
            src_ref: pair.into(),
        })
    }
}
