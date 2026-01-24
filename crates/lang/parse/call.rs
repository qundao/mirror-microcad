// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_syntax::ast;
use crate::{ord_map::*, parse::*, parser::*, syntax::*};

impl Parse for Call {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::call);
        let mut inner = pair.inner();
        let first = inner.next().expect("Expected qualified name");

        Ok(Call {
            name: QualifiedName::parse(first)?,
            argument_list: crate::find_rule!(pair, argument_list)?,
            src_ref: pair.clone().into(),
        })
    }
}

impl FromAst for Call {
    type AstNode = ast::Call;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Call {
            src_ref: context.src_ref(&node.span),
            name: QualifiedName::from_ast(&node.name, context)?,
            argument_list: ArgumentList::from_ast(&node.arguments, context)?,
        })
    }
}

impl Parse for ArgumentList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::argument_list);
        let mut argument_list = ArgumentList(Refer::new(OrdMap::default(), pair.clone().into()));

        match pair.as_rule() {
            Rule::argument_list => {
                for pair in pair.inner() {
                    match pair.as_rule() {
                        Rule::named_argument | Rule::expression => {
                            argument_list
                                .try_push(Argument::parse(pair)?)
                                .map_err(ParseError::DuplicateArgument)?;
                        }
                        Rule::COMMENT => {}
                        rule => unreachable!("Expected argument, found {rule:?}"),
                    }
                }

                Ok(argument_list)
            }
            rule => {
                unreachable!("ArgumentList::parse expected argument list, found {rule:?}")
            }
        }
    }
}

impl FromAst for ArgumentList {
    type AstNode = ast::ArgumentList;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let mut argument_list = ArgumentList(Refer::new(OrdMap::default(), context.src_ref(&node.span)));
        for arg in &node.arguments {
            argument_list
                .try_push(Argument::from_ast(arg, context)?)
                .map_err(ParseError::DuplicateArgument)?;
        }
        Ok(argument_list)
    }
}

impl Parse for Argument {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::named_argument => {
                let mut inner = pair.inner();
                let first = inner.next().expect(INTERNAL_PARSE_ERROR);
                let second = inner.next().expect(INTERNAL_PARSE_ERROR);

                Ok(Argument {
                    id: Some(Identifier::parse(first)?),
                    expression: Expression::parse(second)?,
                    src_ref: pair.src_ref(),
                })
            }
            Rule::expression => Ok(Argument {
                id: None,
                expression: Expression::parse(pair.clone())?,
                src_ref: pair.into(),
            }),
            rule => unreachable!("Argument::parse expected argument, found {rule:?}"),
        }
    }
}

impl FromAst for Argument {
    type AstNode = ast::Argument;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Argument {
            id: node.name().map(|name| Identifier::from_ast(name, context)).transpose()?,
            src_ref: context.src_ref(node.span()),
            expression: Expression::from_ast(node.value(), context)?
        })
    }
}

impl Parse for MethodCall {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(MethodCall {
            name: pair.find(Rule::qualified_name).expect(INTERNAL_PARSE_ERROR),
            argument_list: pair.find(Rule::argument_list).unwrap_or_default(),
            src_ref: pair.clone().into(),
        })
    }
}

#[test]
fn call() {
    use crate::{parser::*, syntax::*};
    use pest::Parser as _;

    let pair = Pair::new(
        Parser::parse(Rule::call, "foo(1, 2, bar = 3, baz = 4)")
            .expect("test error")
            .next()
            .expect("test error"),
        0,
    );

    let call = Call::parse(pair).expect("test error");

    assert_eq!(call.name, "foo".into());
    assert_eq!(call.argument_list.len(), 4);

    // Count named arguments
    let named = call
        .argument_list
        .iter()
        .filter(|arg| arg.id.is_some())
        .count();
    assert_eq!(named, 2);
}
