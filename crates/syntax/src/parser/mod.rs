// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod error;
mod helpers;
mod simplify;

use crate::Span;
use crate::ast::*;
use crate::parser::error::{ParseErrorKind, Rich};
use crate::parser::helpers::*;
use crate::parser::simplify::simplify_unary_op;
use crate::tokens::*;

use chumsky::input::{Input, MappedInput};
use chumsky::prelude::*;
use chumsky::{Parser, extra, select_ref};
use compact_str::CompactString;
pub use error::ParseError;
use helpers::ParserExt;
use std::str::FromStr;

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
) -> Result<SourceFile, Vec<ParseError>> {
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

fn parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, SourceFile, Extra<'tokens>> {
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
            Token::KeywordType
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

    let block_recovery = ignore_till_matched_curly().map_with(|_, e| StatementList {
        span: e.span(),
        statements: Vec::default(),
        tail: None,
        extras: ItemExtras::default(),
    });

    let block = whitespace_parser()
        .or_not()
        .ignore_then(statement_list_parser.clone())
        .then_maybe_whitespace()
        .delimited_with_spanned_error(
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
        )
        .recover_with(via_parser(block_recovery))
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
        Token::Unit(unit) = e => SingleType {
            span: e.span(),
            name: unit.as_ref().into()
        },
        Token::SigilQuote = e => SingleType {
            span: e.span(),
            name: r#"""#.into()
        },
    }
    .labelled("quantity type")
    .boxed();

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
            .then(single_type.clone().or_not())
            .with_extras()
            .try_map_with(|((literal, ty), extras), e| {
                let literal = match (literal, ty) {
                    (LiteralKind::Float(float), Some(ty)) => {
                        LiteralKind::Quantity(QuantityLiteral {
                            span: e.span(),
                            value: float.value,
                            ty,
                        })
                    }
                    (LiteralKind::Integer(int), Some(ty)) => {
                        LiteralKind::Quantity(QuantityLiteral {
                            span: e.span(),
                            value: int.value as f64,
                            ty,
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
        Token::OperatorSubtract => UnaryOperator::Minus,
        Token::OperatorAdd => UnaryOperator::Plus,
        Token::OperatorNot => UnaryOperator::Not,
    }
    .labelled("unary operator")
    .boxed();

    let doc_comment = select_ref! {
        Token::DocComment(comment) => comment,
    }
    .then_whitespace()
    .repeated()
    .at_least(1)
    .collect::<Vec<_>>()
    .map_with(|lines, e| Comment {
        span: e.span(),
        lines: lines.into_iter().map(|s| s.as_ref().into()).collect(),
    })
    .labelled("doc-comment")
    .or_not()
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
    .map_with(|_, e| {
        (
            vec![TupleItem {
                span: e.span(),
                name: None,
                value: Expression::Error(e.span()),
                extras: ItemExtras::default(),
            }],
            ItemExtras::default(),
        )
    });

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
        .then_maybe_whitespace()
        .then(
            whitespace_parser()
                .or_not()
                .ignore_then(tuple_body.clone())
                .with_extras()
                .then_maybe_whitespace()
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
                .recover_with(via_parser(tuple_recovery.clone().map_with(|_, e| {
                    ArgumentList {
                        span: e.span(),
                        extras: ItemExtras::default(),
                        arguments: Vec::new(),
                    }
                }))),
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
                    span: e.span(),
                    extras,
                    attributes,
                    expression,
                },
            )
            .map(Statement::Expression)
            .boxed();

        let assignment_qualifier = select_ref! {
            Token::KeywordConst => AssignmentQualifier::Const,
            Token::KeywordProp => AssignmentQualifier::Prop,
        }
        .then_whitespace()
        .or_not()
        .boxed();

        let assignment_inner = doc_comment
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.then_whitespace().or_not())
            .then(assignment_qualifier)
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
                |(((((((doc, attributes), visibility), qualifier), name), ty), value), extras),
                 e| {
                    Assignment {
                        span: e.span(),
                        extras,
                        doc,
                        attributes,
                        visibility,
                        qualifier,
                        name,
                        value: Box::new(value),
                        ty,
                    }
                },
            )
            .boxed();

        let assignment = assignment_inner
            .clone()
            .map(Statement::Assignment)
            .labelled("assignment");

        attribute_parser.define({
            let attribute_command = assignment_inner
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

        let comment = comment_parser()
            .map(Statement::Comment)
            .then_maybe_whitespace()
            .labelled("comment")
            .boxed();

        let arguments_inner = whitespace_parser()
            .or_not()
            .ignore_then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(
                just(Token::SigilColon)
                    .then_maybe_whitespace()
                    .ignore_then(type_parser.clone())
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .then(
                just(Token::OperatorAssignment)
                    .then_maybe_whitespace()
                    .ignore_then(expression_parser.clone())
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .with_extras()
            .map_with(|(((name, ty), default), extras), e| ArgumentDefinition {
                span: e.span(),
                extras,
                name,
                ty,
                default,
            })
            .then_maybe_whitespace()
            .separated_by(just(Token::SigilComma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .boxed();

        let arguments = arguments_inner
            .then_maybe_whitespace()
            .with_extras()
            .map_with(|(arguments, extras), e| ArgumentsDefinition {
                span: e.span(),
                extras,
                arguments,
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
                            kind: "function arguments",
                            close_token: Token::SigilCloseBracket,
                        },
                    )
                },
            )
            .boxed();

        let module = doc_comment
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.then_whitespace().or_not())
            .then(just(Token::KeywordMod).map_with(|_, e| e.span()))
            .then_whitespace()
            .then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(
                block
                    .clone()
                    .map(Some)
                    .or(just(Token::SigilSemiColon).map(|_| None)),
            )
            .with_extras()
            .map_with(
                |((((((doc, attributes), visibility), keyword_span), name), body), extras), e| {
                    Statement::Module(ModuleDefinition {
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

        let use_statement = visibility
            .then_whitespace()
            .or_not()
            .then_ignore(just(Token::KeywordUse))
            .then_whitespace()
            .then(use_parts)
            .then(
                whitespace_parser()
                    .then(just(Token::KeywordAs))
                    .then_whitespace()
                    .ignore_then(identifier_parser.clone().recover_with(via_parser(
                        recovery_expect_any().map_with(|_, e| Identifier {
                            span: e.span(),
                            name: CompactString::default(),
                        }),
                    )))
                    .or_not(),
            )
            .with_extras()
            .map_with(|(((visibility, name), use_as), extras), e| {
                Statement::Use(UseStatement {
                    span: e.span(),
                    extras,
                    visibility,
                    name,
                    use_as,
                })
            })
            .boxed();

        let workspace_kind = select_ref! {
            Token::KeywordSketch => WorkbenchKind::Sketch,
            Token::KeywordPart => WorkbenchKind::Part,
            Token::KeywordOp => WorkbenchKind::Op,
        }
        .boxed();

        let init = doc_comment
            .clone()
            .then(just(Token::KeywordInit).map_with(|_, e| e.span()))
            .then_maybe_whitespace()
            .then(arguments.clone())
            .then_maybe_whitespace()
            .then(block.clone())
            .with_extras()
            .map_with(|((((doc, keyword_span), arguments), body), extras), e| {
                Statement::Init(InitDefinition {
                    span: e.span(),
                    keyword_span,
                    extras,
                    doc,
                    arguments,
                    body,
                })
            })
            .boxed();
        let workspace = doc_comment
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.then_whitespace().or_not())
            .then(workspace_kind.map_with(|kind, e| (kind, e.span())))
            .then_whitespace()
            .then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(arguments.clone())
            .then_maybe_whitespace()
            .then(block.clone())
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
                        arguments,
                        body,
                    })
                },
            )
            .boxed();

        let return_statement = just(Token::KeywordReturn)
            .then_maybe_whitespace()
            .ignore_then(expression_parser.clone().or_not())
            .with_extras()
            .map_with(|(value, extras), e| {
                Statement::Return(Return {
                    span: e.span(),
                    extras,
                    value,
                })
            })
            .boxed();

        let function = doc_comment
            .clone()
            .then(visibility.then_whitespace().or_not())
            .then(just(Token::KeywordFn).map_with(|_, e| e.span()))
            .then_whitespace()
            .then(identifier_parser.clone())
            .then_maybe_whitespace()
            .then(arguments.clone())
            .then_maybe_whitespace()
            .then(
                just(Token::SigilSingleArrow)
                    .then_maybe_whitespace()
                    .ignore_then(type_parser.clone())
                    .then_maybe_whitespace()
                    .or_not(),
            )
            .then(block.clone())
            .with_extras()
            .map_with(
                |(
                    ((((((doc, visibility), keyword_span), name), arguments), return_type), body),
                    extras,
                ),
                 e| {
                    Statement::Function(FunctionDefinition {
                        span: e.span(),
                        keyword_span,
                        extras,
                        doc,
                        visibility,
                        name,
                        arguments,
                        return_type,
                        body,
                    })
                },
            )
            .boxed();

        let if_expression = attribute_parser
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

        let doc_statement = select_ref! {
            Token::DocComment(comment) => comment,
        }
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map_with(|lines, e| Comment {
            span: e.span(),
            lines: lines.into_iter().map(|s| s.as_ref().into()).collect(),
        })
        .labelled("doc-comment")
        .map(Statement::Comment)
        .boxed();

        let inner_doc_statement = select_ref! {
            Token::InnerDocComment(comment) => String::from(comment.as_ref()),
        }
        .then_maybe_whitespace()
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map_with(|lines, e| Comment {
            span: e.span(),
            lines,
        })
        .labelled("inner doc-comment")
        .map(Statement::InnerDocComment)
        .boxed();

        let not_assigment = whitespace_parser()
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
            .then_ignore(not_assigment.clone())
            .try_map_with(|kind, e| {
                Err::<(), _>(Rich::custom(
                    e.span(),
                    ParseErrorKind::ReservedKeyword(kind),
                ))
            })
            .ignored()
            .recover_with(via_parser(
                reserved_keyword
                    .then_ignore(not_assigment)
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
            .or(assignment)
            .or(expression)
            .then_maybe_whitespace()
            .boxed();

        let without_semi = function
            .or(inner_doc_statement)
            .or(doc_statement)
            .or(init)
            .or(workspace)
            .or(module)
            .or(comment)
            .or(if_expression)
            .boxed();

        without_semi
            .or(reserved_keyword_statement)
            .or(with_semi.then_ignore(just(Token::SigilSemiColon).labelled("semicolon")))
            .labelled("statement")
    });

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
            .map(Statement::Expression)
            .map(Box::new)
            .or_not()
            .then_maybe_whitespace()
            .boxed();

        whitespace_parser()
            .or_not()
            .ignore_then(statement_parser)
            .then_maybe_whitespace()
            .repeated()
            .collect::<Vec<_>>()
            .then(trailing_expr)
            .with_extras()
            .map_with(|((statements, tail), extras), e| StatementList {
                span: e.span(),
                extras,
                statements,
                tail,
            })
            .then_maybe_whitespace()
            .boxed()
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
            .then_maybe_whitespace()
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
            .then_maybe_whitespace()
            .then_ignore(just(Token::SigilDoubleDot))
            .then_maybe_whitespace()
            .then(array_item.clone())
            .then_maybe_whitespace()
            .with_extras()
            .delimited_by(
                just(Token::SigilOpenSquareBracket).then_maybe_whitespace(),
                just(Token::SigilCloseSquareBracket),
            )
            .then(single_type.clone().or_not())
            .map_with(|(((start, end), extras), ty), e| {
                Expression::ArrayRange(ArrayRangeExpression {
                    span: e.span(),
                    extras,
                    start: Box::new(start),
                    end: Box::new(end),
                    ty,
                })
            })
            .labelled("array range")
            .boxed();

        let array_list = array_item
            .clone()
            .then_maybe_whitespace()
            .separated_by(just(Token::SigilComma).then_maybe_whitespace())
            .allow_trailing()
            .collect::<Vec<_>>()
            .with_extras()
            .delimited_by(
                just(Token::SigilOpenSquareBracket).then_maybe_whitespace(),
                just(Token::SigilCloseSquareBracket),
            )
            .then(single_type.clone().or_not())
            .map_with(|((items, extras), ty), e| {
                Expression::ArrayList(ArrayListExpression {
                    span: e.span(),
                    extras,
                    items,
                    ty,
                })
            })
            .labelled("array")
            .boxed();

        let block_expression = block
            .clone()
            .map(Expression::Block)
            .labelled("block expression")
            .boxed();

        if_inner.define(
            just(Token::KeywordIf)
                .then_whitespace()
                .ignore_then(expression_parser.clone())
                .then_maybe_whitespace()
                .then(block.clone())
                .then_maybe_whitespace()
                .then(
                    just(Token::KeywordElse)
                        .then_maybe_whitespace()
                        .ignore_then(if_inner.clone())
                        .map(Box::new)
                        .or_not(),
                )
                .then(
                    just(Token::KeywordElse)
                        .then_maybe_whitespace()
                        .ignore_then(block.clone())
                        .or_not(),
                )
                .with_extras()
                .map_with(
                    |((((condition, body), next_if), else_body), extras), e| If {
                        span: e.span(),
                        extras,
                        condition: Box::new(condition),
                        body,
                        next_if,
                        else_body,
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
            .or(block_expression)
            .or(if_expression)
            .or(qualified_name_expr)
            .boxed();

        let access_attribute = just(Token::SigilHash)
            .ignore_then(identifier_parser.clone())
            .map(Element::Attribute)
            .labelled("attribute access")
            .boxed();

        let access_tuple = just(Token::SigilDot)
            .ignore_then(identifier_parser.clone())
            .map(Element::Tuple)
            .labelled("tuple access")
            .boxed();

        let access_method = just(Token::SigilDot)
            .ignore_then(call_inner)
            .map(Element::Method)
            .labelled("method call")
            .boxed();

        let access_array = expression_parser
            .clone()
            .delimited_by(
                just(Token::SigilOpenSquareBracket),
                just(Token::SigilCloseSquareBracket),
            )
            .map(Box::new)
            .map(Element::ArrayElement)
            .labelled("array access")
            .boxed();

        let access_item = access_attribute
            .or(access_method)
            .or(access_tuple)
            .or(access_array)
            .boxed();

        let element_access = base
            .clone()
            .foldl_with(
                whitespace_parser()
                    .or_not()
                    .ignore_then(access_item)
                    .repeated(),
                |value, element, e| {
                    Expression::ElementAccess(ElementAccess {
                        span: e.span(),
                        value: value.into(),
                        element,
                    })
                },
            )
            .labelled("element access")
            .boxed();

        let unary_expression = unary_operator_parser
            .then_maybe_whitespace()
            .then(element_access.clone())
            .with_extras()
            .map_with(|((op, rhs), extras), e| UnaryOperation {
                span: e.span(),
                extras,
                operation: op,
                rhs: rhs.into(),
            })
            .map(simplify_unary_op)
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
        .map_with(move |statements, ex| SourceFile {
            span: ex.span(),
            statements,
        })
}
