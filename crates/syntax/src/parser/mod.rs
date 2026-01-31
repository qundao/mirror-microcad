mod error;
mod helpers;

use crate::Span;
use crate::ast::*;
use crate::tokens::*;
use chumsky::error::Rich;
use chumsky::input::{Input, MappedInput};
use chumsky::prelude::*;
use chumsky::{Parser, extra, select_ref};
pub use error::ParseError;
use helpers::ParserExt;
use std::str::FromStr;

type Extra<'tokens> = extra::Err<Rich<'tokens, Token<'tokens>, Span>>;

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

pub fn input<'input, 'tokens>(
    input: &'input [SpannedToken<Token<'tokens>>],
) -> ParserInput<'input, 'tokens> {
    let end = input.last().map(|t| t.span.end).unwrap_or_default();
    Input::map(input, end..end, map_token_input)
}

pub fn parse<'tokens>(
    tokens: &'tokens [SpannedToken<Token<'tokens>>],
) -> Result<SourceFile, Vec<ParseError>> {
    parser()
        .parse(input(tokens))
        .into_result()
        .map_err(|errors| errors.into_iter().map(ParseError::new).collect())
}

fn parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, SourceFile, Extra<'tokens>> {
    let mut statement_list_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    let mut type_parser = Recursive::declare();
    let mut attribute_parser = Recursive::declare();
    let mut if_inner = Recursive::declare();

    let semi_recovery = none_of(Token::SigilSemiColon).repeated().ignored();

    let block_recovery = just(Token::SigilOpenCurlyBracket)
        .then(
            none_of(Token::SigilCloseCurlyBracket)
                .repeated()
                .then(just(Token::SigilCloseCurlyBracket)),
        )
        .map(|_| StatementList {
            span: 0..0,
            statements: vec![Statement::Error],
            tail: None,
            extras: ItemExtras::default(),
        });

    let block = statement_list_parser
        .clone()
        .delimited_by(
            just(Token::SigilOpenCurlyBracket),
            just(Token::SigilCloseCurlyBracket),
        )
        .recover_with(via_parser(block_recovery))
        .boxed();

    let identifier_parser = select_ref! { Token::Identifier(ident) = e => Identifier {
        span: e.span(),
        name: ident.as_ref().into(),
    } }
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

    let comment_inner = select_ref! {
        Token::SingleLineComment(comment)= e => Comment {
            span: e.span(),
            comment: comment.as_ref().into()
        },
        Token::MultiLineComment(comment) = e => Comment {
            span: e.span(),
            comment: comment.as_ref().into()
        }
    };

    type_parser.define({
        let single = single_type.clone().map(Type::Single);
        let array = type_parser
            .clone()
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

        let tuple = identifier_parser
            .clone()
            .then_ignore(just(Token::SigilColon))
            .or_not()
            .then(type_parser.clone())
            .separated_by(just(Token::SigilComma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::SigilOpenBracket),
                just(Token::SigilCloseBracket),
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
            .map_with(|((literal, ty), extras), e| {
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
                    (_, Some(_)) => {
                        LiteralKind::Error(LiteralError {
                            span: e.span(),
                            kind: LiteralErrorKind::Untypable,
                        })
                    }
                    (literal, None) => literal,
                };
                Literal { literal, extras }
            })
            .labelled("literal")
            .boxed()
    };

    let binary_operator_parser = select_ref! {
        Token::OperatorAdd => Operator::Add,
        Token::OperatorSubtract => Operator::Subtract,
        Token::OperatorMultiply => Operator::Multiply,
        Token::OperatorDivide => Operator::Divide,
        Token::OperatorUnion => Operator::Union,
        Token::OperatorIntersect => Operator::Intersect,
        Token::OperatorPowerXor => Operator::PowerXor,
        Token::OperatorGreaterThan => Operator::GreaterThan,
        Token::OperatorLessThan => Operator::LessThan,
        Token::OperatorGreaterEqual => Operator::GreaterEqual,
        Token::OperatorLessEqual => Operator::LessEqual,
        Token::OperatorNear => Operator::Near,
        Token::OperatorEqual => Operator::Equal,
        Token::OperatorNotEqual => Operator::NotEqual,
        Token::OperatorAdd => Operator::Add,
        Token::OperatorAnd => Operator::And,
        Token::OperatorOr => Operator::Or,
        Token::OperatorXor => Operator::Xor,
    }
    .labelled("binary operator")
    .boxed();

    let unary_operator_parser = select_ref! {
        Token::OperatorSubtract => UnaryOperator::Minus,
        Token::OperatorAdd => UnaryOperator::Plus,
        Token::OperatorNot => UnaryOperator::Not,
    }
    .labelled("unary operator")
    .boxed();

    let doc_comment = select_ref! {
        Token::DocComment(comment )= e => Comment {
            span: e.span(),
            comment: comment.as_ref().into()
        },
    }
    .labelled("doc-comment")
    .or_not()
    .boxed();

    let tuple_recovery = just(Token::SigilOpenBracket)
        .then(
            none_of(Token::SigilCloseBracket)
                .repeated()
                .then(just(Token::SigilCloseBracket)),
        )
        .map_with(|_, e| {
            (
                vec![TupleItem {
                    span: e.span(),
                    name: None,
                    value: Expression::Error,
                    extras: ItemExtras::default(),
                }],
                ItemExtras::default(),
            )
        });

    let tuple_body = identifier_parser
        .clone()
        .then_ignore(just(Token::OperatorAssignment))
        .or_not()
        .then(expression_parser.clone())
        .with_extras()
        .map_with(|((name, value), extras), e| TupleItem {
            span: e.span(),
            extras,
            name,
            value,
        })
        .separated_by(just(Token::SigilComma))
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
                .delimited_by(
                    just(Token::SigilOpenBracket),
                    just(Token::SigilCloseBracket),
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
            Token::KeywordConst => AssigmentQualifier::Const,
            Token::KeywordProp => AssigmentQualifier::Prop,
        }
        .or_not()
        .boxed();

        let assignment_inner = doc_comment
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.or_not())
            .then(assignment_qualifier)
            .then(identifier_parser.clone())
            .then(
                just(Token::SigilColon)
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then_ignore(just(Token::OperatorAssignment))
            .then(
                expression_parser
                    .clone()
                    .recover_with(via_parser(semi_recovery.clone().map(|_| Expression::Error))),
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
                        value,
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
                .or(qualified_name.clone().map(AttributeCommand::Ident));

            just(Token::SigilHash)
                .ignore_then(attribute_command.delimited_by(
                    just(Token::SigilOpenSquareBracket),
                    just(Token::SigilCloseSquareBracket),
                ))
                .with_extras()
                .map_with(|(command, extras), e| Attribute {
                    span: e.span(),
                    extras,
                    command,
                })
                .labelled("attribute")
                .repeated()
                .collect::<Vec<Attribute>>()
                .boxed()
        });

        let comment = comment_inner
            .clone()
            .map(Statement::Comment)
            .labelled("comment")
            .boxed();

        let arguments_inner = identifier_parser
            .clone()
            .then(
                just(Token::SigilColon)
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then(
                just(Token::OperatorAssignment)
                    .ignore_then(literal_parser.clone())
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
            .separated_by(just(Token::SigilComma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .boxed();

        let arguments = arguments_inner
            .with_extras()
            .map_with(|(arguments, extras), e| ArgumentsDefinition {
                span: e.span(),
                extras,
                arguments,
            })
            .delimited_by(
                just(Token::SigilOpenBracket),
                just(Token::SigilCloseBracket),
            )
            .boxed();

        let module = doc_comment
            .clone()
            .then(attribute_parser.clone())
            .then(visibility.or_not())
            .then_ignore(just(Token::KeywordMod))
            .then(identifier_parser.clone())
            .then(
                block
                    .clone()
                    .map(Some)
                    .or(just(Token::SigilSemiColon).map(|_| None)),
            )
            .with_extras()
            .map_with(
                |(((((doc, attributes), visibility), name), body), extras), e| {
                    Statement::Module(ModuleDefinition {
                        span: e.span(),
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

        let use_parts = identifier_parser
            .clone()
            .map(UseStatementPart::Identifier)
            .or(just(Token::OperatorMultiply).map_with(|_, e| UseStatementPart::Glob(e.span())))
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
            .clone()
            .or_not()
            .then_ignore(just(Token::KeywordUse))
            .then(use_parts)
            .then(
                just(Token::KeywordAs)
                    .ignore_then(identifier_parser.clone())
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
            Token::KeywordSketch => WorkspaceKind::Sketch,
            Token::KeywordPart => WorkspaceKind::Part,
            Token::KeywordOp => WorkspaceKind::Op,
        }
        .boxed();

        let init = doc_comment
            .clone()
            .then_ignore(just(Token::KeywordInit))
            .then(arguments.clone())
            .then(block.clone())
            .with_extras()
            .map_with(|(((doc, arguments), body), extras), e| {
                Statement::Init(InitDefinition {
                    span: e.span(),
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
            .then(visibility.or_not())
            .then(workspace_kind)
            .then(identifier_parser.clone())
            .then(arguments.clone())
            .then(block.clone())
            .with_extras()
            .map_with(
                |(((((((doc, attributes), visibility), kind), name), arguments), body), extras),
                 e| {
                    Statement::Workbench(WorkbenchDefinition {
                        span: e.span(),
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
            .ignore_then(expression_parser.clone())
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
            .then(visibility.or_not())
            .then_ignore(just(Token::KeywordFn))
            .then(identifier_parser.clone())
            .then(arguments.clone())
            .then(
                just(Token::SigilSingleArrow)
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then(block.clone())
            .with_extras()
            .map_with(
                |((((((doc, visibility), name), arguments), return_type), body), extras), e| {
                    Statement::Function(FunctionDefinition {
                        span: e.span(),
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

        let with_semi = assignment
            .or(return_statement)
            .or(use_statement)
            .or(expression)
            .boxed();

        let without_semi = function
            .or(init)
            .or(workspace)
            .or(module)
            .or(comment)
            .or(if_expression)
            .boxed();

        without_semi
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
            .map(Box::new)
            .or_not();
        statement_parser
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
                        "unclosed string",
                    ))
                })
                .recover_with(via_parser(semi_recovery.clone().map(|_| Expression::Error))),
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

        let format_accuracy = select_ref!(
            Token::StringFormatPrecision(precision) = e => {
                usize::from_str(&precision[1..]).map_err(|err| (err, e.span()))
            }
        )
        .labelled("string format accuracy");
        let format_width = select_ref!(
            Token::StringFormatWidth(width) = e => {
                usize::from_str(&width[1..]).map_err(|err| (err, e.span()))
            }
        )
        .labelled("string format width");
        let string_format_part = expression_parser
            .clone()
            .then(format_width.or_not())
            .then(format_accuracy.or_not())
            .with_extras()
            .delimited_by(
                just(Token::StringFormatOpen),
                just(Token::StringFormatClose),
            )
            .map_with(
                |(((expression, width), accuracy), extras), e| StringExpression {
                    span: e.span(),
                    extras,
                    expression,
                    accuracy,
                    width,
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

        let tuple = tuple_body
            .clone()
            .with_extras()
            .delimited_by(
                just(Token::SigilOpenBracket),
                just(Token::SigilCloseBracket),
            )
            .recover_with(via_parser(tuple_recovery))
            .map_with(|(values, extras), e| {
                Expression::Tuple(TupleExpression {
                    span: e.span(),
                    extras,
                    values,
                })
            })
            .labelled("tuple");

        let bracketed = expression_parser.clone().delimited_by(
            just(Token::SigilOpenBracket),
            just(Token::SigilCloseBracket),
        );

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
                just(Token::SigilOpenSquareBracket),
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
            .separated_by(just(Token::SigilComma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .with_extras()
            .delimited_by(
                just(Token::SigilOpenSquareBracket),
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
                .ignore_then(expression_parser.clone())
                .then(block.clone())
                .then(
                    just(Token::KeywordElse)
                        .ignore_then(if_inner.clone())
                        .map(Box::new)
                        .or_not(),
                )
                .then(just(Token::KeywordElse).ignore_then(block.clone()).or_not())
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

        let base = literal
            .or(string_format)
            .or(call)
            .or(qualified_name_expr)
            .or(marker)
            .or(bracketed)
            .or(tuple)
            .or(array_range)
            .or(array_list)
            .or(block_expression)
            .or(if_expression)
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
            .foldl_with(access_item.repeated(), |value, element, e| {
                Expression::ElementAccess(ElementAccess {
                    span: e.span(),
                    value: value.into(),
                    element,
                })
            })
            .labelled("element access")
            .boxed();

        let unary_expression = unary_operator_parser
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

        let binary_expression = binary_param
            .clone()
            .foldl_with(
                binary_operator_parser.then(binary_param.clone()).repeated(),
                |lhs, (op, rhs), e| {
                    Expression::BinaryOperation(BinaryOperation {
                        span: e.span(),
                        lhs: lhs.into(),
                        operation: op,
                        rhs: rhs.into(),
                    })
                },
            )
            .boxed();

        binary_expression.labelled("expression").boxed()
    });

    statement_list_parser.map_with(move |statements, ex| SourceFile {
        span: ex.span(),
        statements,
    })
}
