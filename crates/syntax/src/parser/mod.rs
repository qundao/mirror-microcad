// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod error;
mod helpers;

use crate::Span;
use crate::ast::*;
use crate::parser::error::{ParseErrorKind, Rich};
use crate::parser::helpers::*;
use crate::tokens::*;

use chumsky::input::{Input, MappedInput};
use chumsky::prelude::*;
use chumsky::{Parser, extra, select_ref};
pub use error::ParseError;
use helpers::ParserExt;
use std::str::FromStr;

use compact_str::ToCompactString;

type Error<'tokens> = Rich<'tokens, Token<'tokens>, Span, ParseErrorKind>;
type Extra<'tokens> = extra::Err<Error<'tokens>>;

pub fn map_token_input<'a, 'token>(
    spanned: &'a SpannedToken<Token<'token>>,
) -> (&'a Token<'token>, &'a Span) {
    (&spanned.token, &spanned.span)
}

type InputMap<'input, 'token> =
    fn(&'input SpannedToken<Token<'token>>) -> (&'input Token<'token>, &'input Span);

type ParserInput<'input, 'token> = MappedInput<
    'input,
    Token<'token>,
    Span,
    &'input [SpannedToken<Token<'token>>],
    InputMap<'input, 'token>,
>;

fn input<'input, 'tokens>(
    input: &'input [SpannedToken<Token<'tokens>>],
) -> ParserInput<'input, 'tokens> {
    let end = input.last().map(|t| t.span.end).unwrap_or_default();
    Input::map(input, end..end, map_token_input)
}

/// Build an abstract syntax tree from a list of tokens
pub fn parse<'tokens>(
    tokens: &'tokens [SpannedToken<Token<'tokens>>],
) -> Result<Source, Vec<ParseError>> {
    parser()
        .parse(input(tokens))
        .into_result()
        .map_err(|errors| errors.into_iter().map(ParseError::new).collect())
}

const STRUCTURAL_TOKENS: &[Token] = &[
    Token::SigilOpenCurlyBracket,
    Token::SigilCloseCurlyBracket,
    Token::SigilOpenBracket,
    Token::SigilCloseBracket,
    Token::SigilOpenSquareBracket,
    Token::SigilCloseSquareBracket,
    Token::SigilSemiColon,
];

fn parser<'tokens>() -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, Source, Extra<'tokens>>
{
    let mut statement_list_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    let mut type_parser = Recursive::declare();
    let mut attribute_parser = Recursive::declare();
    let mut if_inner = Recursive::declare();

    let semi_recovery = none_of(Token::SigilSemiColon).repeated().ignored();

    let reserved_keyword = select_ref! {
        token @ (
            Token::KeywordPlugin |
            Token::KeywordAssembly |
            Token::KeywordMaterial |
            Token::KeywordUnit |
            Token::KeywordEnum |
            Token::KeywordStruct |
            Token::KeywordMatch |
            Token::KeywordType |
            Token::KeywordExtern
        ) => token.kind(),
    }
    .boxed();
    let keyword = select_ref! {
        token @ (
            Token::KeywordMod |
            Token::KeywordPart |
            Token::KeywordSketch |
            Token::KeywordOp |
            Token::KeywordFn |
            Token::KeywordIf |
            Token::KeywordElse |
            Token::KeywordUse |
            Token::KeywordAs |
            Token::KeywordReturn |
            Token::KeywordPub |
            Token::KeywordConst |
            Token::KeywordProp |
            Token::KeywordInit
        ) => token.kind(),
    }
    .boxed();

    statement_list_parser.define({
        let trailing_expr = attribute_parser
            .clone()
            .then(expression_parser.clone())
            .with_extras()
            .map_with(
                |((attributes, expression), extras), e| ExpressionStatement {
                    span: e.span(),
                    extras,
                    attributes,
                    expression,
                },
            )
            .map(Box::new)
            .or_not()
            .boxed();

        whitespace_parser()
            .or_not()
            .ignore_then(statement_parser.clone())
            .then(trailing_extras_parser())
            .repeated()
            .collect::<Vec<(Statement, TrailingExtras)>>()
            .then(trailing_expr)
            .with_extras()
            .map_with(|((statements, tail), extras), e| StatementList {
                span: e.span(),
                extras,
                statements,
                tail,
            })
            .boxed()
    });

    let block_recovery =
        ignore_till_matched_curly().map_with(|_, e| StatementList::dummy(e.span()));

    let body = whitespace_parser()
        .or_not()
        .ignore_then(statement_list_parser.clone().delimited_with_spanned_error(
            just(Token::SigilOpenCurlyBracket),
            just(Token::SigilCloseCurlyBracket),
            |err: Error, open, end| {
                Rich::custom(
                    err.span().clone(),
                    ParseErrorKind::UnclosedBracket {
                        open,
                        end,
                        kind: "code block",
                        close_token: Token::SigilCloseCurlyBracket,
                    },
                )
            },
        ))
        .recover_with(via_parser(block_recovery))
        .map_with(|statements, e| Body {
            span: e.span(),
            statements,
        })
        .boxed();

    let identifier_parser = select_ref! { Token::Identifier(ident) = e => Identifier {
        span: e.span(),
        name: ident.as_ref().into(),
    } }
    .or(reserved_keyword
        .clone()
        .validate(|kind, e, emitter| {
            emitter.emit(Rich::custom(
                e.span(),
                ParseErrorKind::ReservedKeywordAsIdentifier(kind),
            ));
            kind
        })
        .map_with(|kind, e| Identifier {
            span: e.span(),
            name: kind.into(),
        }))
    .or(keyword
        .validate(|kind, e, emitter| {
            emitter.emit(Rich::custom(
                e.span(),
                ParseErrorKind::KeywordAsIdentifier(kind),
            ));
            kind
        })
        .map_with(|kind, e| Identifier {
            span: e.span(),
            name: kind.into(),
        }))
    .labelled("identifier")
    .boxed();

    let qualified_name = identifier_parser
        .clone()
        .separated_by(just(Token::SigilDoubleColon))
        .at_least(1)
        .collect::<Vec<_>>()
        .with_extras()
        .map_with(|(parts, extras), e| QualifiedName {
            span: e.span(),
            parts,
            extras,
        })
        .labelled("qualified name")
        .boxed();

    let single_type = select_ref! {
        Token::Identifier(ident) = e => SingleType {
            span: e.span(),
            name: ident.as_ref().into()
        },
    }
    .labelled("quantity type")
    .boxed();

    let unit = select_ref! {
        Token::Identifier(ident) = e => Unit {
            span: e.span(),
            name: ident.as_ref().into()
        },
        Token::Unit(unit) = e => Unit {
            span: e.span(),
            name: unit.as_ref().into()
        },
        Token::SigilQuote = e => Unit {
            span: e.span(),
            name: r#"""#.into()
        },
    };

    type_parser.define({
        let single = single_type.clone().map(Type::Single);
        let array = whitespace_parser()
            .or_not()
            .ignore_then(type_parser.clone())
            .then_maybe_whitespace()
            .delimited_by(
                just(Token::SigilOpenSquareBracket),
                just(Token::SigilCloseSquareBracket),
            )
            .map_with(|inner, e| {
                Type::Array(ArrayType {
                    span: e.span(),
                    inner: Box::new(inner),
                })
            })
            .labelled("array type")
            .boxed();

        let tuple = whitespace_parser()
            .or_not()
            .ignore_then(identifier_parser.clone())
            .then_ignore(just(Token::SigilColon))
            .or_not()
            .then_maybe_whitespace()
            .then(type_parser.clone())
            .then_maybe_whitespace()
            .separated_by(just(Token::SigilComma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .then_maybe_whitespace()
            .delimited_with_spanned_error(
                just(Token::SigilOpenBracket),
                just(Token::SigilCloseBracket),
                |err: Error, open, end| {
                    Rich::custom(
                        err.span().clone(),
                        ParseErrorKind::UnclosedBracket {
                            open,
                            end,
                            kind: "tuple type",
                            close_token: Token::SigilCloseBracket,
                        },
                    )
                },
            )
            .map_with(|inner, e| {
                Type::Tuple(TupleType {
                    span: e.span(),
                    inner,
                })
            })
            .labelled("tuple type")
            .boxed();

        single.or(array).or(tuple).labelled("type").boxed()
    });

    let literal_parser = {
        let single_value = select_ref! {
            Token::LiteralFloat(x) = e => {
                match f64::from_str(x) {
                    Ok(value) => LiteralKind::Float(FloatLiteral {
                        value,
                        raw: x.to_compact_string(),
                        span: e.span(),
                    }),
                    Err(err) => LiteralKind::Error(LiteralError {
                        span: e.span(),
                        kind: err.into(),
                    })
                }
            },
            Token::LiteralInt(x) = e => {
                match i64::from_str(x) {
                    Ok(value) => LiteralKind::Integer(IntegerLiteral {
                    value,
                    raw: x.to_compact_string(),
                    span: e.span(),
                }),
                    Err(err) => LiteralKind::Error(LiteralError {
                        span: e.span(),
                        kind: err.into(),
                    })
                }
            },
            Token::LiteralString(content) = e => {
                LiteralKind::String(StringLiteral {
                    span: e.span(),
                    content: content.as_ref().into(),
                })
            },
            Token::LiteralBool(value) = e => {
                LiteralKind::Bool(BoolLiteral {
                    span: e.span(),
                    value: *value,
                })
            },
        }
        .boxed();

        single_value
            .then(unit.clone().or_not())
            .with_extras()
            .try_map_with(|((literal, ty), extras), e| {
                let literal = match (literal, ty) {
                    (LiteralKind::Float(float), Some(unit)) => {
                        LiteralKind::Quantity(QuantityLiteral {
                            span: e.span(),
                            value: float.value,
                            raw: float.raw,
                            unit,
                        })
                    }
                    (LiteralKind::Integer(int), Some(unit)) => {
                        LiteralKind::Quantity(QuantityLiteral {
                            span: e.span(),
                            value: int.value as f64,
                            raw: int.raw,
                            unit,
                        })
                    }
                    (_, Some(_)) => LiteralKind::Error(LiteralError {
                        span: e.span(),
                        kind: LiteralErrorKind::Untypable,
                    }),
                    (literal, None) => literal,
                };
                Ok(Literal {
                    span: e.span(),
                    literal,
                    extras,
                })
            })
            .labelled("literal")
            .boxed()
    };

    let unary_operator_parser = select_ref! {
        Token::OperatorSubtract = e => UnaryOperator { span: e.span(), operation: UnaryOperatorType::Minus },
        Token::OperatorAdd = e => UnaryOperator { span: e.span(), operation: UnaryOperatorType::Plus },
        Token::OperatorNot = e => UnaryOperator { span: e.span(), operation: UnaryOperatorType::Not },
    }
    .labelled("unary operator")
    .boxed();

    // Parse for a DocBlock, starting with `///`
    let doc_block = select_ref! {
        Token::DocComment(comment) => comment.to_string(),
    }
    .then_whitespace()
    .repeated()
    .collect::<Vec<_>>()
    .map_with(|lines, e| DocBlock {
        span: e.span(),
        lines,
    })
    .labelled("doc block")
    .boxed();

    let tuple_recovery = nested_delimiters(
        Token::SigilOpenBracket,
        Token::SigilCloseBracket,
        [
            (
                Token::SigilOpenSquareBracket,
                Token::SigilCloseSquareBracket,
            ),
            (Token::SigilOpenCurlyBracket, Token::SigilCloseCurlyBracket),
        ],
        |_| (),
    )
    .map_with(|_, e| (vec![TupleItem::dummy(e.span())], ItemExtras::default()));

    let tuple_body = identifier_parser
        .clone()
        .then_maybe_whitespace()
        .then_ignore(just(Token::OperatorAssignment).then_maybe_whitespace())
        .or_not()
        .then(expression_parser.clone())
        .with_extras()
        .map_with(|((name, value), extras), e| TupleItem {
            span: e.span(),
            extras,
            name,
            value,
        })
        .then_maybe_whitespace()
        .separated_by(just(Token::SigilComma).then_maybe_whitespace())
        .allow_trailing()
        .collect::<Vec<_>>()
        .boxed();

    let call_inner = qualified_name
        .clone()
        .then(
            tuple_body
                .clone()
                .with_extras()
                .map_with(|(arguments, extras), e| ArgumentList {
                    span: e.span(),
                    extras,
                    arguments: arguments
                        .into_iter()
                        .map(|item| match item.name {
                            Some(name) => Argument::Named(NamedArgument {
                                span: item.span,
                                extras: item.extras,
                                name,
                                value: item.value,
                            }),
                            None => Argument::Unnamed(UnnamedArgument {
                                span: item.span,
                                extras: item.extras,
                                value: item.value,
                            }),
                        })
                        .collect::<Vec<_>>(),
                })
                .labelled("function arguments")
                .delimited_with_spanned_error(
                    just(Token::SigilOpenBracket),
                    just(Token::SigilCloseBracket),
                    |err: Error, open, end| {
                        Rich::custom(
                            err.span().clone(),
                            ParseErrorKind::UnclosedBracket {
                                open,
                                end,
                                kind: "function arguments",
                                close_token: Token::SigilCloseBracket,
                            },
                        )
                    },
                )
                .recover_with(via_parser(
                    tuple_recovery
                        .clone()
                        .map_with(|_, e| ArgumentList::dummy(e.span())),
                )),
        )
        .with_extras()
        .map_with(|((name, arguments), extras), e| Call {
            span: e.span(),
            extras,
            name,
            arguments,
        })
        .boxed();

    statement_parser.define({
        let visibility = select_ref! {
            Token::KeywordPub => Visibility::Public,
        }
        .labelled("visibility");

        let expression = attribute_parser
            .clone()
            .then(expression_parser.clone())
            .with_extras()
            .map_with(
                |((attributes, expression), extras), e| ExpressionStatement {
                    span: e.span(), // FIXME: This should only return the span of attributes and expression
                    extras,
                    attributes,
                    expression,
                },
            )
            .map(Statement::Expression)
            .boxed();

        let local_assignment_inner = attribute_parser
            .clone()
            .then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(
                just(Token::SigilColon)
                    .then_maybe_whitespace()
                    .ignore_then(type_parser.clone())
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .then_ignore(just(Token::OperatorAssignment))
            .then_maybe_whitespace()
            .then(
                expression_parser.clone().recover_with(via_parser(
                    semi_recovery
                        .clone()
                        .map_with(|_, e| Expression::Error(e.span())),
                )),
            )
            .with_extras()
            .map_with(
                |((((attributes, name), ty), value), extras), e| LocalAssignment {
                    span: e.span(),
                    extras,
                    attributes,
                    name,
                    value: Box::new(value),
                    ty,
                },
            )
            .boxed();

        let local_assignment = local_assignment_inner
            .clone()
            .map(Statement::LocalAssignment)
            .labelled("local assignment");

        let const_assignment_inner = doc_block
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.then_whitespace().or_not())
            .then(just(Token::KeywordConst).map_with(|_, e| e.span()))
            .then_maybe_whitespace()
            .then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(
                just(Token::SigilColon)
                    .then_maybe_whitespace()
                    .ignore_then(type_parser.clone())
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .then_ignore(just(Token::OperatorAssignment))
            .then_maybe_whitespace()
            .then(
                expression_parser.clone().recover_with(via_parser(
                    semi_recovery
                        .clone()
                        .map_with(|_, e| Expression::Error(e.span())),
                )),
            )
            .with_extras()
            .map_with(
                |(
                    ((((((doc, attributes), visibility), keyword_span), name), ty), value),
                    extras,
                ),
                 e| {
                    ConstAssignment {
                        span: e.span(),
                        keyword_span,
                        extras,
                        doc,
                        attributes,
                        visibility,
                        name,
                        value: Box::new(value),
                        ty,
                    }
                },
            )
            .boxed();

        let const_assignment = const_assignment_inner
            .clone()
            .map(Statement::Const)
            .labelled("const assignment");

        // A pub assignment without the `const` keyword will eventually become a const assignment
        let pub_assignment_inner = doc_block
            .clone()
            .then(attribute_parser.clone())
            .then(just(Token::KeywordPub).map_with(|_, e| e.span()))
            .then_maybe_whitespace()
            .then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(
                just(Token::SigilColon)
                    .then_maybe_whitespace()
                    .ignore_then(type_parser.clone())
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .then_ignore(just(Token::OperatorAssignment))
            .then_maybe_whitespace()
            .then(
                expression_parser.clone().recover_with(via_parser(
                    semi_recovery
                        .clone()
                        .map_with(|_, e| Expression::Error(e.span())),
                )),
            )
            .with_extras()
            .map_with(
                |((((((doc, attributes), keyword_span), name), ty), value), extras), e| {
                    ConstAssignment {
                        span: e.span(),
                        keyword_span,
                        extras,
                        doc,
                        attributes,
                        visibility: Some(Visibility::Public),
                        name,
                        value: Box::new(value),
                        ty,
                    }
                },
            )
            .boxed();

        let pub_assignment = pub_assignment_inner
            .clone()
            .map(Statement::Const)
            .labelled("pub const assignment");

        let property_assignment_inner = doc_block
            .clone()
            .then(attribute_parser.clone())
            .then(just(Token::KeywordProp).map_with(|_, e| e.span()))
            .then_maybe_whitespace()
            .then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(
                just(Token::SigilColon)
                    .then_maybe_whitespace()
                    .ignore_then(type_parser.clone())
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .then_ignore(just(Token::OperatorAssignment))
            .then_maybe_whitespace()
            .then(
                expression_parser.clone().recover_with(via_parser(
                    semi_recovery
                        .clone()
                        .map_with(|_, e| Expression::Error(e.span())),
                )),
            )
            .with_extras()
            .map_with(
                |((((((doc, attributes), keyword_span), name), ty), value), extras), e| {
                    PropertyAssignment {
                        span: e.span(),
                        keyword_span,
                        extras,
                        doc,
                        attributes,
                        name,
                        value: Box::new(value),
                        ty,
                    }
                },
            )
            .boxed();

        let property_assignment = property_assignment_inner
            .clone()
            .map(Statement::Property)
            .labelled("property assignment");

        attribute_parser.define({
            let attribute_command = local_assignment_inner
                .clone()
                .map(AttributeCommand::Assignment)
                .or(call_inner.clone().map(AttributeCommand::Call))
                .or(identifier_parser.clone().map(AttributeCommand::Ident));

            just(Token::SigilHash)
                .ignore_then(just(Token::OperatorNot).or_not().map(|opt| opt.is_some()))
                .then(
                    attribute_command
                        .separated_by(just(Token::SigilComma).then_maybe_whitespace())
                        .at_least(1)
                        .collect::<Vec<_>>()
                        .then_maybe_whitespace()
                        .delimited_by(
                            just(Token::SigilOpenSquareBracket),
                            just(Token::SigilCloseSquareBracket),
                        ),
                )
                .then_whitespace()
                .with_extras()
                .map_with(|((is_inner, commands), extras), e| Attribute {
                    span: e.span(),
                    is_inner,
                    extras,
                    commands,
                })
                .labelled("attribute")
                .repeated()
                .collect::<Vec<Attribute>>()
                .boxed()
        });

        let parameter_list_inner = whitespace_parser()
            .or_not()
            .ignore_then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(
                just(Token::SigilColon)
                    .then_maybe_whitespace()
                    .ignore_then(
                        type_parser.clone().recover_with(via_parser(
                            recovery_expect_any_except(&[
                                Token::SigilComma,
                                Token::OperatorAssignment,
                                Token::SigilCloseBracket,
                            ])
                            .map_with(|_, e| Type::dummy(e.span())),
                        )),
                    )
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .then(
                just(Token::OperatorAssignment)
                    .then_maybe_whitespace()
                    .ignore_then(
                        expression_parser.clone().recover_with(via_parser(
                            recovery_expect_any_except(&[
                                Token::SigilComma,
                                Token::SigilCloseBracket,
                            ])
                            .map_with(|_, e| Expression::Error(e.span())),
                        )),
                    )
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .with_extras()
            .map_with(|(((name, ty), default), extras), e| Parameter {
                span: e.span(),
                extras,
                name,
                ty,
                default,
            })
            .separated_by(just(Token::SigilComma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .boxed();

        let parameter_list = parameter_list_inner
            .then_maybe_whitespace()
            .with_extras()
            .map_with(|(parameters, extras), e| ParameterList {
                span: e.span(),
                extras,
                parameters,
            })
            .delimited_with_spanned_error(
                just(Token::SigilOpenBracket),
                just(Token::SigilCloseBracket),
                |err: Error, open, end| {
                    Rich::custom(
                        err.span().clone(),
                        ParseErrorKind::UnclosedBracket {
                            open,
                            end,
                            kind: "function parameters",
                            close_token: Token::SigilCloseBracket,
                        },
                    )
                },
            )
            .recover_with(via_parser(
                ignore_till_matched_brackets()
                    .or(none_of(STRUCTURAL_TOKENS).repeated())
                    .map_with(|_, e| ParameterList::dummy(e.span())),
            ))
            .boxed();

        let inline_module = doc_block
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.then_whitespace().or_not())
            .then(just(Token::KeywordMod).map_with(|_, e| e.span()))
            .then_whitespace()
            .then(
                identifier_parser.clone().recover_with(via_parser(
                    recovery_expect_any_except(&[Token::SigilOpenCurlyBracket])
                        .map_with(|_, e| Identifier::dummy(e.span())),
                )),
            )
            .then_maybe_whitespace()
            .then(body.clone())
            .with_extras()
            .map_with(
                |((((((doc, attributes), visibility), keyword_span), name), body), extras), e| {
                    Statement::InlineModule(InlineModule {
                        span: e.span(),
                        keyword_span,
                        extras,
                        doc,
                        attributes,
                        visibility,
                        name,
                        body,
                    })
                },
            )
            .boxed();

        let file_module = doc_block
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.then_whitespace().or_not())
            .then(just(Token::KeywordMod).map_with(|_, e| e.span()))
            .then_whitespace()
            .then(
                identifier_parser.clone().recover_with(via_parser(
                    recovery_expect_any_except(&[Token::SigilOpenCurlyBracket])
                        .map_with(|_, e| Identifier::dummy(e.span())),
                )),
            )
            .with_extras()
            .map_with(
                |(((((doc, attributes), visibility), keyword_span), name), extras), e| {
                    Statement::FileModule(FileModule {
                        span: e.span(),
                        keyword_span,
                        extras,
                        doc,
                        attributes,
                        visibility,
                        name,
                    })
                },
            )
            .boxed();

        let use_part = identifier_parser
            .clone()
            .map(UseStatementPart::Identifier)
            .or(just(Token::OperatorMultiply).map_with(|_, e| UseStatementPart::Glob(e.span())))
            .recover_with(via_parser(
                recovery_expect_any_except(&[Token::SigilDoubleColon])
                    .map_with(|_, e| UseStatementPart::Error(e.span())),
            ))
            .boxed();

        let use_parts = use_part
            .separated_by(just(Token::SigilDoubleColon))
            .at_least(1)
            .collect::<Vec<_>>()
            .with_extras()
            .map_with(|(parts, extras), e| UseName {
                span: e.span(),
                extras,
                parts,
            })
            .boxed();

        let use_statement = attribute_parser
            .clone()
            .then(visibility.then_whitespace().or_not())
            .then(just(Token::KeywordUse).map_with(|_, e| e.span()))
            .then_whitespace()
            .then(use_parts)
            .then(
                select_ref! {
                    Token::KeywordAs => (),
                }
                .then_whitespace()
                .ignore_then(identifier_parser.clone().recover_with(via_parser(
                    recovery_expect_any().map_with(|_, e| Identifier::dummy(e.span())),
                )))
                .or_not(),
            )
            .with_extras()
            .map_with(
                |(((((attributes, visibility), keyword_span), name), use_as), extras), e| {
                    Statement::Use(UseStatement {
                        span: e.span(),
                        attributes,
                        visibility,
                        keyword_span,
                        extras,
                        name,
                        use_as,
                    })
                },
            )
            .boxed();

        let workbench_kind = select_ref! {
            Token::KeywordSketch => WorkbenchKind::Sketch,
            Token::KeywordPart => WorkbenchKind::Part,
            Token::KeywordOp => WorkbenchKind::Op,
        }
        .boxed();

        let init = doc_block
            .clone()
            .then(attribute_parser.clone())
            .then(just(Token::KeywordInit).map_with(|_, e| e.span()))
            .then_maybe_whitespace()
            .then(parameter_list.clone())
            .then(body.clone())
            .with_extras()
            .map_with(
                |(((((doc, attributes), keyword_span), arguments), body), extras), e| {
                    Statement::Init(InitDefinition {
                        span: e.span(),
                        keyword_span,
                        extras,
                        doc,
                        attributes,
                        parameters: arguments,
                        body,
                    })
                },
            )
            .boxed();
        let workbench = doc_block
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.then_whitespace().or_not())
            .then(workbench_kind.map_with(|kind, e| (kind, e.span())))
            .then_whitespace()
            .then(
                identifier_parser.clone().recover_with(via_parser(
                    recovery_expect_any_except(&[
                        Token::SigilOpenCurlyBracket,
                        Token::SigilOpenBracket,
                    ])
                    .map_with(|_, e| Identifier::dummy(e.span())),
                )),
            )
            .then_maybe_whitespace()
            .then(parameter_list.clone())
            .then_maybe_whitespace()
            .then(body.clone())
            .with_extras()
            .map_with(
                |(
                    (
                        (
                            ((((doc, attributes), visibility), (kind, keyword_span)), name),
                            arguments,
                        ),
                        body,
                    ),
                    extras,
                ),
                 e| {
                    Statement::Workbench(WorkbenchDefinition {
                        span: e.span(),
                        keyword_span,
                        extras,
                        kind,
                        doc,
                        attributes,
                        visibility,
                        name,
                        plan: arguments,
                        body,
                    })
                },
            )
            .boxed();

        let return_statement = just(Token::KeywordReturn)
            .map_with(|_, e| e.span())
            .then_maybe_whitespace()
            .then(expression_parser.clone().or_not())
            .with_extras()
            .map_with(|((keyword_span, value), extras), e| {
                Statement::Return(Return {
                    span: e.span(),
                    keyword_span,
                    extras,
                    value,
                })
            })
            .boxed();

        let function = doc_block
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.then_whitespace().or_not())
            .then(just(Token::KeywordFn).map_with(|_, e| e.span()))
            .then_whitespace()
            .then(
                identifier_parser.clone().recover_with(via_parser(
                    recovery_expect_any_except(&[
                        Token::SigilOpenCurlyBracket,
                        Token::SigilOpenBracket,
                    ])
                    .map_with(|_, e| Identifier::dummy(e.span())),
                )),
            )
            .then_maybe_whitespace()
            .then(parameter_list.clone())
            .then_maybe_whitespace()
            .then(
                just(Token::SigilSingleArrow)
                    .then_maybe_whitespace()
                    .ignore_then(type_parser.clone())
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .then(body.clone())
            .with_extras()
            .map_with(
                |(
                    (
                        (
                            (((((doc, attributes), visibility), keyword_span), name), arguments),
                            return_type,
                        ),
                        body,
                    ),
                    extras,
                ),
                 e| {
                    Statement::Function(FunctionDefinition {
                        span: e.span(),
                        keyword_span,
                        extras,
                        doc,
                        attributes,
                        visibility,
                        name,
                        parameters: arguments,
                        return_type,
                        body,
                    })
                },
            )
            .boxed();

        let if_statement = attribute_parser
            .clone()
            .then(if_inner.clone().map(Expression::If))
            .with_extras()
            .map_with(|((attributes, expression), extras), e| {
                Statement::Expression(ExpressionStatement {
                    span: e.span(),
                    extras,
                    attributes,
                    expression,
                })
            })
            .labelled("if statement")
            .boxed();

        let body_statement = attribute_parser
            .clone()
            .then(body.clone().map(Expression::Body))
            .with_extras()
            .map_with(|((attributes, expression), extras), e| {
                Statement::Expression(ExpressionStatement {
                    span: e.span(),
                    extras,
                    attributes,
                    expression,
                })
            })
            .labelled("body statement")
            .boxed();

        let inner_doc_comment = select_ref! {
            Token::InnerDocComment(comment) => comment.to_string(),
        }
        .labelled("inner doc-block")
        .map_with(|line, e| {
            Statement::InnerDocComment(InnerDocComment {
                span: e.span(),
                line,
            })
        })
        .boxed();

        let not_assignment = whitespace_parser()
            .or_not()
            .then(none_of([
                Token::OperatorAssignment,
                Token::SigilDoubleColon,
            ]))
            .ignored()
            .or(just(Token::SigilSemiColon).ignored())
            .rewind()
            .boxed();

        let reserved_keyword_statement = reserved_keyword
            .clone()
            .then_ignore(not_assignment.clone())
            .try_map_with(|kind, e| {
                Err::<(), _>(Rich::custom(
                    e.span(),
                    ParseErrorKind::ReservedKeyword(kind),
                ))
            })
            .ignored()
            .recover_with(via_parser(
                reserved_keyword
                    .then_ignore(not_assignment)
                    .clone()
                    .ignore_then(
                        none_of(STRUCTURAL_TOKENS)
                            .repeated()
                            .then_ignore(ignore_till_matched_curly())
                            .or(ignore_till_semi().then_ignore(just(Token::SigilSemiColon))),
                    ),
            ))
            .map_with(|_, e| Statement::Error(e.span()))
            .boxed();

        let with_semi = return_statement
            .or(use_statement)
            .or(const_assignment)
            .or(pub_assignment)
            .or(file_module)
            .or(property_assignment)
            .or(local_assignment)
            .or(expression)
            .then_ignore(
                just(Token::SigilSemiColon)
                    .labelled("semicolon")
                    .ignored()
                    .recover_with(via_parser(
                        none_of(STRUCTURAL_TOKENS)
                            .repeated()
                            .then_ignore(just(Token::SigilSemiColon)),
                    )),
            )
            .boxed();

        let without_semi = function
            .or(inner_doc_comment)
            .or(init)
            .or(workbench)
            .or(inline_module)
            .or(if_statement)
            .or(body_statement)
            .boxed();

        with_semi
            .or(reserved_keyword_statement)
            .or(without_semi)
            .boxed()
            .labelled("statement")
    });

    expression_parser.define({
        let unclosed_string = select_ref! {
            Token::Error(LexerError::UnclosedString(_)) => (),
        }
        .ignore_then(
            semi_recovery
                .clone()
                .try_map_with(|_, e| {
                    let span: Span = e.span();
                    Err::<Expression, _>(Rich::custom(
                        (span.start - 1)..span.end,
                        ParseErrorKind::UnterminatedString,
                    ))
                })
                .recover_with(via_parser(
                    semi_recovery
                        .clone()
                        .map_with(|_, e| Expression::Error(e.span())),
                )),
        )
        .labelled("unclosed string")
        .boxed();

        let literal = literal_parser
            .map(Expression::Literal)
            .labelled("literal")
            .boxed()
            .or(unclosed_string);

        let marker = just(Token::SigilAt)
            .ignore_then(identifier_parser.clone())
            .map(Expression::Marker)
            .labelled("marker")
            .boxed();

        let string_content_part = select_ref! {
            Token::StringContent(content) = e => StringPart::Content(StringLiteral {
                span: e.span(),
                content: content.as_ref().into(),
            }),
            Token::Character(char) = e => StringPart::Char(StringCharacter {
                span: e.span(),
                character: *char,
            }),
        }
        .labelled("string content")
        .boxed();

        let format_precision = select_ref!(
            Token::StringFormatPrecision(precision) = e => {
                u32::from_str(&precision[1..]).map_err(|err| (err, e.span()))
            }
        );
        let format_width = select_ref!(
            Token::StringFormatWidth(width) = e => {
                u32::from_str(&width[1..]).map_err(|err| (err, e.span()))
            }
        );
        let format_spec = format_width
            .or_not()
            .then(format_precision.or_not())
            .map_with(|(width, precision), e| StringFormatSpecification {
                span: e.span(),
                width,
                precision,
            })
            .labelled("string format specification")
            .boxed();

        let string_format_part = expression_parser
            .clone()
            .then(format_spec)
            .with_extras()
            .delimited_by(
                just(Token::StringFormatOpen),
                just(Token::StringFormatClose),
            )
            .map_with(
                |((expression, specification), extras), e| StringExpression {
                    span: e.span(),
                    extras,
                    expression: Box::new(expression),
                    specification: Box::new(specification),
                },
            )
            .map(StringPart::Expression)
            .labelled("string format expression")
            .boxed();
        let string_part = string_content_part
            .or(string_format_part)
            .labelled("format string content");

        let string_format = string_part
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just(Token::FormatStringStart), just(Token::FormatStringEnd))
            .with_extras()
            .map_with(|(parts, extras), e| FormatString {
                span: e.span(),
                extras,
                parts,
            })
            .map(Expression::String)
            .boxed();

        let tuple = whitespace_parser()
            .or_not()
            .ignore_then(tuple_body.clone())
            .with_extras()
            .delimited_with_spanned_error(
                just(Token::SigilOpenBracket),
                just(Token::SigilCloseBracket),
                |err: Error, open, end| {
                    Rich::custom(
                        err.span().clone(),
                        ParseErrorKind::UnclosedBracket {
                            open,
                            end,
                            kind: "tuple",
                            close_token: Token::SigilCloseBracket,
                        },
                    )
                },
            )
            .map_with(|(values, extras), e| {
                Expression::Tuple(TupleExpression {
                    span: e.span(),
                    extras,
                    values,
                })
            })
            .labelled("tuple");

        let bracketed = expression_parser
            .clone()
            .then_maybe_whitespace()
            .delimited_with_spanned_error(
                just(Token::SigilOpenBracket).then_maybe_whitespace(),
                just(Token::SigilCloseBracket),
                |err: Error, open, end| {
                    Rich::custom(
                        err.span().clone(),
                        ParseErrorKind::UnclosedBracket {
                            open,
                            end,
                            kind: "bracketed expression",
                            close_token: Token::SigilCloseBracket,
                        },
                    )
                },
            )
            .map_with(|expression, e| Expression::Bracketed(Box::new(expression), e.span()))
            .boxed();

        let array_item = expression_parser
            .clone()
            .with_extras()
            .map_with(|(expression, extras), e| ArrayItem {
                span: e.span(),
                extras,
                expression,
            })
            .boxed();

        let array_range = array_item
            .clone()
            .then_ignore(just(Token::SigilDoubleDot))
            .then(array_item.clone())
            .with_extras()
            .delimited_by(
                just(Token::SigilOpenSquareBracket).then_maybe_whitespace(),
                just(Token::SigilCloseSquareBracket),
            )
            .then(unit.clone().or_not())
            .map_with(|(((start, end), extras), unit), e| {
                Expression::ArrayRange(ArrayRangeExpression {
                    span: e.span(),
                    extras,
                    start: Box::new(start),
                    end: Box::new(end),
                    unit,
                })
            })
            .labelled("array range")
            .boxed();

        let array_list = array_item
            .clone()
            .separated_by(just(Token::SigilComma).then_maybe_whitespace())
            .allow_trailing()
            .collect::<Vec<_>>()
            .with_extras()
            .delimited_by(
                just(Token::SigilOpenSquareBracket).then_maybe_whitespace(),
                just(Token::SigilCloseSquareBracket),
            )
            .then(unit.clone().or_not())
            .map_with(|((items, extras), unit), e| {
                Expression::ArrayList(ArrayListExpression {
                    span: e.span(),
                    extras,
                    items,
                    unit,
                })
            })
            .labelled("array")
            .boxed();

        let body_expression = body
            .clone()
            .map(Expression::Body)
            .labelled("body expression")
            .boxed();

        if_inner.define(
            just(Token::KeywordIf)
                .map_with(|_, e| e.span())
                .then_whitespace()
                .then(expression_parser.clone())
                .then_maybe_whitespace()
                .then(body.clone())
                .then_maybe_whitespace()
                .then(
                    just(Token::KeywordElse)
                        .map_with(|_, e| e.span())
                        .then_maybe_whitespace()
                        .then(if_inner.clone())
                        .map(|(span, inner)| (span, Box::new(inner)))
                        .or_not(),
                )
                .then(
                    just(Token::KeywordElse)
                        .map_with(|_, e| e.span())
                        .then_maybe_whitespace()
                        .then(body.clone())
                        .or_not(),
                )
                .with_extras()
                .map_with(
                    |(((((if_span, condition), body), next_if), else_body), extras), e| {
                        let (next_if_span, next_if) = next_if
                            .map(|(span, if_expr)| (Some(span), Some(if_expr)))
                            .unwrap_or((None, None));
                        let (else_span, else_body) = else_body
                            .map(|(span, body)| (Some(span), Some(body)))
                            .unwrap_or((None, None));
                        If {
                            span: e.span(),
                            if_span,
                            extras,
                            condition: Box::new(condition),
                            body,
                            next_if_span,
                            next_if,
                            else_span,
                            else_body,
                        }
                    },
                )
                .boxed(),
        );
        let if_expression = if_inner
            .map(Expression::If)
            .labelled("if expression")
            .boxed();

        let qualified_name_expr = identifier_parser
            .clone()
            .map_with(|ident, e| QualifiedName {
                span: e.span(),
                parts: vec![ident],
                extras: ItemExtras::default(),
            })
            .foldl_with(
                just(Token::SigilDoubleColon)
                    .ignore_then(identifier_parser.clone())
                    .repeated(),
                |mut acc, part, _| {
                    acc.span.end = part.span.end;
                    acc.parts.push(part);
                    acc
                },
            )
            .with_extras()
            .map(|(mut name, extras)| {
                name.extras = extras;
                name
            })
            .map(Expression::QualifiedName)
            .boxed();

        let call = call_inner
            .clone()
            .map(Expression::Call)
            .labelled("method call");

        let bracket_based = bracketed.or(tuple).recover_with(via_parser(
            tuple_recovery
                .clone()
                .map_with(|_, e| Expression::Error(e.span())),
        ));

        let base = literal
            .or(string_format)
            .or(call)
            .or(marker)
            .or(bracket_based)
            .or(array_range)
            .or(array_list)
            .or(body_expression)
            .or(if_expression)
            .or(qualified_name_expr)
            .boxed();

        let access_attribute = just(Token::SigilHash)
            .ignore_then(identifier_parser.clone())
            .map(ElementInner::Attribute)
            .labelled("attribute access")
            .boxed();

        let access_tuple = just(Token::SigilDot)
            .ignore_then(identifier_parser.clone())
            .map(ElementInner::Tuple)
            .labelled("tuple access")
            .boxed();

        let access_method = just(Token::SigilDot)
            .ignore_then(call_inner)
            .map(ElementInner::Method)
            .labelled("method call")
            .boxed();

        let access_array = expression_parser
            .clone()
            .delimited_by(
                just(Token::SigilOpenSquareBracket),
                just(Token::SigilCloseSquareBracket),
            )
            .map(Box::new)
            .map(ElementInner::ArrayElement)
            .labelled("array access")
            .boxed();

        let access_item = access_attribute
            .or(access_method)
            .or(access_tuple)
            .or(access_array)
            .with_extras()
            .map_with(|(inner, extras), e| Element {
                span: e.span(),
                inner,
                extras,
            })
            .repeated()
            .at_least(1)
            .collect::<Vec<Element>>();

        let element_access = base
            .clone()
            .foldl_with(access_item.repeated(), |value, element_chain, e| {
                Expression::ElementAccess(ElementAccess {
                    span: e.span(),
                    value: value.into(),
                    element_chain,
                })
            })
            .labelled("element access")
            .boxed();

        let unary_expression = unary_operator_parser
            .then_maybe_whitespace()
            .then(element_access.clone())
            .with_extras()
            .map_with(|((op, rhs), extras), e| {
                Expression::UnaryOperation(UnaryOperation {
                    span: e.span(),
                    extras,
                    operation: op,
                    rhs: rhs.into(),
                })
            })
            .boxed();

        let binary_param = element_access.or(unary_expression.clone());

        let near = binop(binary_param, &[Token::OperatorNear]);
        let xor = binop(near, &[Token::OperatorPowerXor, Token::OperatorXor]);
        let union_intersect = binop(xor, &[Token::OperatorUnion, Token::OperatorIntersect]);
        let mul_div = binop(
            union_intersect,
            &[Token::OperatorMultiply, Token::OperatorDivide],
        );
        let add_sub = binop(mul_div, &[Token::OperatorAdd, Token::OperatorSubtract]);
        let less_greater_eq = binop(
            add_sub,
            &[Token::OperatorLessEqual, Token::OperatorGreaterEqual],
        );
        let less_greater = binop(
            less_greater_eq,
            &[Token::OperatorLessThan, Token::OperatorGreaterThan],
        );
        let eq_neq = binop(
            less_greater,
            &[Token::OperatorEqual, Token::OperatorNotEqual],
        );
        let or_and = binop(eq_neq, &[Token::OperatorOr, Token::OperatorAnd]);

        or_and.labelled("expression").boxed()
    });

    statement_list_parser
        .then_ignore(end())
        .map_with(move |statements, ex| Source {
            span: ex.span(),
            statements,
        })
}
