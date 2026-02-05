// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{ord_map::*, parse::*, parser::*};
use microcad_syntax::ast;

/// Short cut to create a `ParameterList` instance
impl Parse for Parameter {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut default_value = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::r#type => {
                    specified_type = Some(TypeAnnotation::parse(pair)?);
                }
                Rule::expression => {
                    default_value = Some(Expression::parse(pair)?);
                }
                rule => {
                    unreachable!(
                        "Unexpected token in parameter: {:?} {:?}",
                        rule,
                        pair.as_span().as_str()
                    );
                }
            }
        }

        if specified_type.is_none() && default_value.is_none() {
            return Err(ParseError::ParameterMissingTypeOrValue(name.clone()));
        }

        Ok(Self {
            id: name,
            specified_type,
            default_value,
            src_ref: pair.into(),
        })
    }
}

impl FromAst for Parameter {
    type AstNode = ast::ArgumentDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Parameter {
            src_ref: context.src_ref(&node.span),
            id: Identifier::from_ast(&node.name, context)?,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| TypeAnnotation::from_ast(ty, context))
                .transpose()?,
            default_value: node
                .default
                .as_ref()
                .map(|def| Expression::from_ast(def, context))
                .transpose()?,
        })
    }
}

impl Parse for ParameterList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);

        let mut parameters: OrdMap<_, _> = Default::default();

        for pair in pair.inner().filter(|p| p.as_rule() == Rule::parameter) {
            parameters
                .try_push(Parameter::parse(pair)?)
                .map_err(ParseError::DuplicateIdentifier)?;
        }

        Ok(ParameterList(Refer::new(parameters, pair.src_ref())))
    }
}

impl FromAst for ParameterList {
    type AstNode = ast::ArgumentsDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let mut parameters: OrdMap<_, _> = Default::default();

        for param in &node.arguments {
            let param = Parameter::from_ast(param, context)?;
            parameters
                .try_push(param)
                .map_err(ParseError::DuplicateIdentifier)?;
        }
        Ok(ParameterList(Refer::new(
            parameters,
            context.src_ref(&node.span),
        )))
    }
}
