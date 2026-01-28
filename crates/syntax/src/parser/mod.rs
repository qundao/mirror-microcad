mod error;

use crate::Span;
use crate::ast::*;
use crate::tokens::*;
use chumsky::error::Rich;
use chumsky::input::{Input, MappedInput};
use chumsky::prelude::*;
use chumsky::{Parser, extra, select_ref};
pub use error::ParseError;
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
    let mut format_string_part_parser = Recursive::declare();
    let mut type_parser = Recursive::declare();
    let mut attribute_parser = Recursive::declare();
    let mut if_inner = Recursive::declare();

    let semi_recovery = none_of(Token::Normal(NormalToken::SigilSemiColon))
        .repeated()
        .ignored();

    let block_recovery = just(Token::Normal(NormalToken::SigilOpenCurlyBracket))
        .then(
            none_of(Token::Normal(NormalToken::SigilCloseCurlyBracket))
                .repeated()
                .then(just(Token::Normal(NormalToken::SigilCloseCurlyBracket))),
        )
        .map(|_| StatementList {
            span: 0..0,
            statements: vec![Statement::Error],
            tail: None,
            tail_comment: None,
        });

    let block = statement_list_parser
        .clone()
        .delimited_by(
            just(Token::Normal(NormalToken::SigilOpenCurlyBracket)),
            just(Token::Normal(NormalToken::SigilCloseCurlyBracket)),
        )
        .recover_with(via_parser(block_recovery))
        .boxed();

    let identifier_parser =
        select_ref! { Token::Normal(NormalToken::Identifier(ident)) = e => Identifier {
            span: e.span(),
            name: ident.as_ref().into(),
        } }
        .labelled("identifier")
        .boxed();

    let qualified_name = identifier_parser
        .clone()
        .separated_by(just(Token::Normal(NormalToken::SigilDoubleColon)))
        .at_least(1)
        .collect::<Vec<_>>()
        .map_with(|parts, e| QualifiedName {
            span: e.span(),
            parts,
        })
        .labelled("qualified name")
        .boxed();

    let single_type = select_ref! {
        Token::Normal(NormalToken::Identifier(ident)) = e => SingleType {
            span: e.span(),
            name: ident.as_ref().into()
        },
        Token::Normal(NormalToken::Unit(unit)) = e => SingleType {
            span: e.span(),
            name: unit.as_ref().into()
        },
        Token::Normal(NormalToken::Quote(QuoteVariant::Unit)) = e => SingleType {
            span: e.span(),
            name: r#"""#.into()
        },
        Token::Error(LexerError::UnclosedString(_)) = e => SingleType {
            span: e.span(),
            name: r#"""#.into()
        }
    }
    .labelled("quantity type")
    .boxed();

    let comment_inner = select_ref! {
        Token::Normal(NormalToken::SingleLineComment(comment) )= e => Comment {
            span: e.span(),
            comment: comment.as_ref().into()
        },
        Token::Normal(NormalToken::MultiLineComment(comment)) = e => Comment {
            span: e.span(),
            comment: comment.as_ref().into()
        }
    };

    type_parser.define({
        let single = single_type.clone().map(Type::Single);
        let array = type_parser
            .clone()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenSquareBracket)),
                just(Token::Normal(NormalToken::SigilCloseSquareBracket)),
            )
            .map_with(|inner, e| {
                Type::Array(ArrayType {
                    span: e.span(),
                    inner: Box::new(inner),
                })
            })
            .boxed();

        let tuple = identifier_parser
            .clone()
            .then_ignore(just(Token::Normal(NormalToken::SigilColon)))
            .or_not()
            .then(type_parser.clone())
            .separated_by(just(Token::Normal(NormalToken::SigilComma)))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenBracket)),
                just(Token::Normal(NormalToken::SigilCloseBracket)),
            )
            .map_with(|inner, e| {
                Type::Tuple(TupleType {
                    span: e.span(),
                    inner,
                })
            })
            .boxed();

        single.or(array).or(tuple).labelled("type").boxed()
    });

    let literal_parser = {
        let single_number = select_ref! {
            Token::Normal(NormalToken::LiteralFloat(x)) = e => {
                match f64::from_str(x) {
                    Ok(value) => Literal::Float(FloatLiteral {
                        value,
                        span: e.span(),
                    }),
                    Err(err) => Literal::Error(LiteralError {
                        span: e.span(),
                        kind: err.into(),
                    })
                }
            },
            Token::Normal(NormalToken::LiteralInt(x)) = e => {
                match i64::from_str(x) {
                    Ok(value) => Literal::Integer(IntegerLiteral {
                    value,
                    span: e.span(),
                }),
                    Err(err) => Literal::Error(LiteralError {
                        span: e.span(),
                        kind: err.into(),
                    })
                }
            },
        }
        .then(just(Token::Normal(NormalToken::SigilDot)).or_not())
        .map_with(|(literal, trailing), e| match (literal, trailing) {
            (Literal::Integer(IntegerLiteral { value, .. }), Some(_)) => {
                Literal::Float(FloatLiteral {
                    value: value as f64,
                    span: e.span(),
                })
            }
            (Literal::Float(FloatLiteral { value, .. }), Some(_)) => Literal::Float(FloatLiteral {
                value,
                span: e.span(),
            }),
            (lit, _) => lit,
        });

        let single_value = select_ref! {
            Token::Normal(NormalToken::Quote(QuoteVariant::String(str_tokens))) = e if is_literal_string(str_tokens) => {
                Literal::String(StringLiteral {
                    span: e.span(),
                    content: get_literal_string(str_tokens).expect("non literal string"),
                })
            },
            Token::Normal(NormalToken::LiteralBoolTrue) = e => {
                Literal::Bool(BoolLiteral {
                    span: e.span(),
                    value: true,
                })
            },
            Token::Normal(NormalToken::LiteralBoolFalse) = e => {
                Literal::Bool(BoolLiteral {
                    span: e.span(),
                    value: false,
                })
            },
        }.or(single_number);

        let quantity = select_ref! {
            Token::Normal(NormalToken::LiteralInt(x)) => x,
            Token::Normal(NormalToken::LiteralFloat(x)) => x,
        }
        .then(single_type.clone())
        .map_with(|(num, ty), e| {
            let value = match f64::from_str(num) {
                Ok(value) => value,
                Err(err) => {
                    return Literal::Error(LiteralError {
                        span: e.span(),
                        kind: err.into(),
                    });
                }
            };
            Literal::Quantity(QuantityLiteral {
                span: e.span(),
                value,
                ty,
            })
        });

        quantity.or(single_value).labelled("literal").boxed()
    };

    let binary_operator_parser = select_ref! {
        Token::Normal(NormalToken::OperatorAdd) => Operator::Add,
        Token::Normal(NormalToken::OperatorSubtract) => Operator::Subtract,
        Token::Normal(NormalToken::OperatorMultiply) => Operator::Multiply,
        Token::Normal(NormalToken::OperatorDivide) => Operator::Divide,
        Token::Normal(NormalToken::OperatorUnion) => Operator::Union,
        Token::Normal(NormalToken::OperatorIntersect) => Operator::Intersect,
        Token::Normal(NormalToken::OperatorPowerXor) => Operator::PowerXor,
        Token::Normal(NormalToken::OperatorGreaterThan) => Operator::GreaterThan,
        Token::Normal(NormalToken::OperatorLessThan) => Operator::LessThan,
        Token::Normal(NormalToken::OperatorGreaterEqual) => Operator::GreaterEqual,
        Token::Normal(NormalToken::OperatorLessEqual) => Operator::LessEqual,
        Token::Normal(NormalToken::OperatorNear) => Operator::Near,
        Token::Normal(NormalToken::OperatorEqual) => Operator::Equal,
        Token::Normal(NormalToken::OperatorNotEqual) => Operator::NotEqual,
        Token::Normal(NormalToken::OperatorAdd) => Operator::Add,
        Token::Normal(NormalToken::OperatorAnd) => Operator::And,
        Token::Normal(NormalToken::OperatorOr) => Operator::Or,
        Token::Normal(NormalToken::OperatorXor) => Operator::Xor,
    }
    .labelled("binary operator")
    .boxed();

    let unary_operator_parser = select_ref! {
        Token::Normal(NormalToken::OperatorSubtract) => UnaryOperator::Minus,
        Token::Normal(NormalToken::OperatorAdd) => UnaryOperator::Plus,
        Token::Normal(NormalToken::OperatorNot) => UnaryOperator::Not,
    }
    .labelled("unary operator")
    .boxed();

    let doc_comment = select_ref! {
        Token::Normal(NormalToken::DocComment(comment) )= e => Comment {
            span: e.span(),
            comment: comment.as_ref().into()
        },
    }
    .labelled("doc-comment")
    .or_not()
    .boxed();

    let tuple_recovery = just(Token::Normal(NormalToken::SigilOpenBracket))
        .then(
            none_of(Token::Normal(NormalToken::SigilCloseBracket))
                .repeated()
                .then(just(Token::Normal(NormalToken::SigilCloseBracket))),
        )
        .map_with(|_, e| {
            vec![TupleItem {
                span: e.span(),
                leading_comment: None,
                name: None,
                value: Expression::Error,
                trailing_comment: None,
            }]
        });

    let tuple_body = comment_inner
        .clone()
        .or_not()
        .then(
            identifier_parser
                .clone()
                .then_ignore(just(Token::Normal(NormalToken::OperatorAssignment)))
                .or_not(),
        )
        .then(expression_parser.clone())
        .then(comment_inner.clone().or_not())
        .map_with(
            |(((leading_comment, name), value), trailing_comment), e| TupleItem {
                span: e.span(),
                leading_comment,
                name,
                value,
                trailing_comment,
            },
        )
        .separated_by(just(Token::Normal(NormalToken::SigilComma)))
        .allow_trailing()
        .collect::<Vec<_>>()
        .boxed();

    let call_inner = qualified_name
        .clone()
        .then(comment_inner
                  .clone()
                  .or_not()
                  .then(tuple_body.clone())
                .map_with(|(leading_comment, arguments), e| ArgumentList {
                    span: e.span(),
                    leading_comment,
                    arguments: arguments
                        .into_iter()
                        .map(|item| match item.name {
                            Some(name) => Argument::Named(NamedArgument {
                                span: item.span,
                                leading_comment: item.leading_comment,
                                name,
                                value: item.value,
                                trailing_comment: item.trailing_comment,
                            }),
                            None => Argument::Unnamed(UnnamedArgument {
                                span: item.span,
                                leading_comment: item.leading_comment,
                                value: item.value,
                                trailing_comment: item.trailing_comment,
                            }),
                        })
                        .collect::<Vec<_>>(),
                })
                .labelled("function arguments")
                .delimited_by(
                    just(Token::Normal(NormalToken::SigilOpenBracket)),
                    just(Token::Normal(NormalToken::SigilCloseBracket)),
                )
                .recover_with(via_parser(tuple_recovery.clone().map_with(|_, e| {
                    ArgumentList {
                        span: e.span(),
                        leading_comment: None,
                        arguments: Vec::new(),
                    }
                }))),
        )
        .map_with(|(name, arguments), e| Call {
            span: e.span(),
            name,
            arguments,
        })
        .boxed();

    statement_parser.define({
        let visibility = select_ref! {
            Token::Normal(NormalToken::KeywordPub) => Visibility::Public,
        }
        .labelled("visibility");

        let expression = attribute_parser
            .clone()
            .then(expression_parser.clone())
            .map_with(|(attributes, expression), e| ExpressionStatement {
                span: e.span(),
                attributes,
                expression,
            })
            .map(Statement::Expression);

        let assignment_qualifier = select_ref! {
            Token::Normal(NormalToken::KeywordConst) => AssigmentQualifier::Const,
            Token::Normal(NormalToken::KeywordProp) => AssigmentQualifier::Prop,
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
                just(Token::Normal(NormalToken::SigilColon))
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then_ignore(just(Token::Normal(NormalToken::OperatorAssignment)))
            .then(
                expression_parser
                    .clone()
                    .recover_with(via_parser(semi_recovery.clone().map(|_| Expression::Error))),
            )
            .map_with(
                |((((((doc, attributes), visibility), qualifier), name), ty), value), e| {
                    Assignment {
                        span: e.span(),
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

            just(Token::Normal(NormalToken::SigilHash))
                .ignore_then(attribute_command.delimited_by(
                    just(Token::Normal(NormalToken::SigilOpenSquareBracket)),
                    just(Token::Normal(NormalToken::SigilCloseSquareBracket)),
                ))
                .then(comment_inner.clone().or_not())
                .map_with(|(command, comment), e| Attribute {
                    span: e.span(),
                    comment,
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

        let arguments = identifier_parser
            .clone()
            .then(
                just(Token::Normal(NormalToken::SigilColon))
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then(
                just(Token::Normal(NormalToken::OperatorAssignment))
                    .ignore_then(literal_parser.clone())
                    .or_not(),
            )
            .map_with(|((name, ty), default), e| ArgumentDefinition {
                span: e.span(),
                name,
                ty,
                default,
            })
            .separated_by(just(Token::Normal(NormalToken::SigilComma)))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenBracket)),
                just(Token::Normal(NormalToken::SigilCloseBracket)),
            )
            .boxed();

        let module = doc_comment
            .clone()
            .then(visibility.or_not())
            .then_ignore(just(Token::Normal(NormalToken::KeywordMod)))
            .then(identifier_parser.clone())
            .then(
                block
                    .clone()
                    .map(Some)
                    .or(just(Token::Normal(NormalToken::SigilSemiColon)).map(|_| None)),
            )
            .map_with(|(((doc, visibility), name), body), e| {
                Statement::Module(ModuleDefinition {
                    span: e.span(),
                    doc,
                    attributes: Vec::new(), // todo
                    visibility,
                    name,
                    body,
                })
            })
            .boxed();

        let use_parts = identifier_parser
            .clone()
            .map(UseStatementPart::Identifier)
            .or(just(Token::Normal(NormalToken::OperatorMultiply))
                .map_with(|_, e| UseStatementPart::Glob(e.span())))
            .separated_by(just(Token::Normal(NormalToken::SigilDoubleColon)))
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|parts, e| UseName {
                span: e.span(),
                parts,
            })
            .boxed();

        let use_statement = visibility
            .clone()
            .or_not()
            .then_ignore(just(Token::Normal(NormalToken::KeywordUse)))
            .then(use_parts)
            .then(
                just(Token::Normal(NormalToken::KeywordAs))
                    .ignore_then(identifier_parser.clone())
                    .or_not(),
            )
            .map_with(|((visibility, name), use_as), e| {
                Statement::Use(UseStatement {
                    span: e.span(),
                    visibility,
                    name,
                    use_as,
                })
            })
            .boxed();

        let workspace_kind = select_ref! {
            Token::Normal(NormalToken::KeywordSketch) => WorkspaceKind::Sketch,
            Token::Normal(NormalToken::KeywordPart) => WorkspaceKind::Part,
            Token::Normal(NormalToken::KeywordOp) => WorkspaceKind::Op,
        }
        .boxed();

        let init = doc_comment
            .clone()
            .then_ignore(just(Token::Normal(NormalToken::KeywordInit)))
            .then(arguments.clone())
            .then(block.clone())
            .map_with(|((doc, arguments), body), e| {
                Statement::Init(InitDefinition {
                    span: e.span(),
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
            .map_with(
                |((((((doc, attributes), visibility), kind), name), arguments), body), e| {
                    Statement::Workbench(WorkbenchDefinition {
                        span: e.span(),
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

        let return_statement = just(Token::Normal(NormalToken::KeywordReturn))
            .ignore_then(expression_parser.clone())
            .map_with(|value, e| {
                Statement::Return(Return {
                    span: e.span(),
                    value,
                })
            })
            .boxed();

        let function = doc_comment
            .clone()
            .then(visibility.or_not())
            .then_ignore(just(Token::Normal(NormalToken::KeywordFn)))
            .then(identifier_parser.clone())
            .then(arguments.clone())
            .then(
                just(Token::Normal(NormalToken::SigilSingleArrow))
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then(block.clone())
            .map_with(
                |(((((doc, visibility), name), arguments), return_type), body), e| {
                    Statement::Function(FunctionDefinition {
                        span: e.span(),
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
            .map_with(|(attributes, expression), e| {
                Statement::Expression(ExpressionStatement {
                    span: e.span(),
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
            .or(with_semi.then_ignore(
                just(Token::Normal(NormalToken::SigilSemiColon)).labelled("semicolon"),
            ))
            .labelled("statement")
    });

    statement_list_parser.define({
        let trailing_expr = attribute_parser
            .clone()
            .then(expression_parser.clone())
            .map_with(|(attributes, expression), e| ExpressionStatement {
                span: e.span(),
                attributes,
                expression,
            })
            .map(Box::new)
            .or_not();
        statement_parser
            .repeated()
            .collect::<Vec<_>>()
            .then(trailing_expr)
            .then(comment_inner.or_not())
            .map_with(|((statements, tail), tail_comment), e| StatementList {
                span: e.span(),
                statements,
                tail,
                tail_comment,
            })
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
                    Err::<Expression, _>(Rich::custom((span.start - 1)..span.end, "unclosed string"))
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

        let marker = just(Token::Normal(NormalToken::SigilAt))
            .ignore_then(identifier_parser.clone())
            .map(Expression::Marker)
            .labelled("marker")
            .boxed();

        let string_format_tokens = select_ref!(
            Token::Normal(NormalToken::Quote(QuoteVariant::String(str_tokens))) if !is_literal_string(str_tokens) => {
                input(str_tokens)
            }
        );

        let string_format_recovery = select_ref!(
            Token::Error(LexerError::UnclosedStringFormat(_span)) => {
                Expression::Error
            }
        );

        let string_format = format_string_part_parser
            .clone()
            .repeated()
            .collect::<Vec<_>>()
            .nested_in(string_format_tokens)
            .map_with(|parts, e| FormatString {
                span: e.span(),
                parts,
            })
            .map(Expression::String)
            .map_err(|e: Rich<'tokens, Token<'tokens>, Span>| {
                Rich::custom(e.span().clone(), "Invalid format string")
            })
            .recover_with(via_parser(string_format_recovery))
            .boxed();

        let tuple = tuple_body.clone()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenBracket)),
                just(Token::Normal(NormalToken::SigilCloseBracket)),
            )
            .recover_with(via_parser(tuple_recovery))
            .map_with(|values, e| {
            Expression::Tuple(TupleExpression {
                span: e.span(),
                values,
            })
        })
            .labelled("tuple");

        let bracketed = expression_parser.clone().delimited_by(
            just(Token::Normal(NormalToken::SigilOpenBracket)),
            just(Token::Normal(NormalToken::SigilCloseBracket)),
        );

        let array_item = comment_inner.or_not().clone()
            .then(expression_parser.clone())
            .then(comment_inner.or_not().clone())
            .map_with(|((leading_comment, expression), trailing_comment), e| ArrayItem {
                span: e.span(),
                leading_comment,
                expression,
                trailing_comment,
            });

        let array_range = array_item
            .clone()
            .then_ignore(just(Token::Normal(NormalToken::SigilDoubleDot)))
            .then(array_item.clone())
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenSquareBracket)),
                just(Token::Normal(NormalToken::SigilCloseSquareBracket)),
            )
            .then(single_type.clone().or_not())
            .map_with(|((start, end), ty), e| {
                Expression::ArrayRange(ArrayRangeExpression {
                    span: e.span(),
                    start: Box::new(start),
                    end: Box::new(end),
                    ty,
                })
            })
            .labelled("array range")
            .boxed();

        let array_list = array_item
            .clone()
            .separated_by(just(Token::Normal(NormalToken::SigilComma)))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenSquareBracket)),
                just(Token::Normal(NormalToken::SigilCloseSquareBracket)),
            )
            .then(single_type.clone().or_not())
            .map_with(|(items, ty), e| {
                Expression::ArrayList(ArrayListExpression {
                    span: e.span(),
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
            just(Token::Normal(NormalToken::KeywordIf))
                .ignore_then(expression_parser.clone())
                .then(block.clone())
                .then(
                    just(Token::Normal(NormalToken::KeywordElse))
                        .ignore_then(if_inner.clone())
                        .map(Box::new)
                        .or_not(),
                )
                .then(
                    just(Token::Normal(NormalToken::KeywordElse))
                        .ignore_then(block.clone())
                        .or_not(),
                )
                .map_with(|(((condition, body), next_if), else_body), e| If {
                    span: e.span(),
                    condition: Box::new(condition),
                    body,
                    next_if,
                    else_body,
                }),
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
            })
            .foldl_with(
                just(Token::Normal(NormalToken::SigilDoubleColon))
                    .ignore_then(identifier_parser.clone())
                    .repeated(),
                |mut acc, part, _| {
                    acc.span.end = part.span.end;
                    acc.parts.push(part);
                    acc
                },
            )
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

        let access_attribute = just(Token::Normal(NormalToken::SigilHash))
            .ignore_then(identifier_parser.clone())
            .map(Element::Attribute)
            .labelled("attribute access")
            .boxed();

        let access_tuple = just(Token::Normal(NormalToken::SigilDot))
            .ignore_then(identifier_parser.clone())
            .map(Element::Tuple)
            .labelled("tuple access")
            .boxed();

        let access_method = just(Token::Normal(NormalToken::SigilDot))
            .ignore_then(call_inner)
            .map(Element::Method)
            .labelled("method call")
            .boxed();

        let access_array = expression_parser
            .clone()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenSquareBracket)),
                just(Token::Normal(NormalToken::SigilCloseSquareBracket)),
            )
            .map(Box::new)
            .map(Element::ArrayElement)
            .labelled("array access")
            .boxed();

        let access_item = access_attribute
            .or(access_method)
            .or(access_tuple)
            .or(access_array);

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
            .map_with(|(op, rhs), e| {
                Expression::UnaryOperation(UnaryOperation {
                    span: e.span(),
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

        binary_expression
            .labelled("expression")
            .boxed()
    });

    format_string_part_parser.define({
        let content = select_ref!(
            Token::String(StringToken::Content(str)) = e => {
                StringPart::Content(StringLiteral {
                    span: e.span(),
                    content: str.as_ref().into(),
                })
            },
            Token::String(StringToken::Escaped(str)) => {
                StringPart::Char(str.chars().nth(2).expect("invalid escaped token"))
            },
            Token::String(StringToken::EscapedCurlyOpen ) => {
                StringPart::Char('{')
            },
            Token::String(StringToken::EscapedCurlyClose) => {
                StringPart::Char('}')
            },
            Token::String(StringToken::BackSlash) => {
                StringPart::Char('\\')
            },
        );

        let format_tokens = select_ref!(
            Token::String(StringToken::FormatStart(args)) => {
                input(args)
            }
        );

        let format_accuracy = select_ref!(
            Token::StringFormat(StringFormatToken::FormatPrecision(precision)) = e => {
                usize::from_str(&precision[1..]).map_err(|err| (err, e.span()))
            }
        );
        let format_width = select_ref!(
            Token::StringFormat(StringFormatToken::FormatWidth(width)) = e => {
                usize::from_str(&width[1..]).map_err(|err| (err, e.span()))
            }
        );

        let format_expr = expression_parser
            .then(format_width.or_not())
            .then(format_accuracy.or_not())
            .nested_in(format_tokens)
            .map_with(|((expr, width), accuracy), e| {
                StringPart::Expression(StringExpression {
                    span: e.span(),
                    expression: expr,
                    accuracy,
                    width,
                })
            })
            .boxed();

        content.or(format_expr)
    });

    statement_list_parser.map_with(move |statements, ex| SourceFile {
        span: ex.span(),
        statements,
    })
}
