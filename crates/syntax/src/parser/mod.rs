mod error;

use crate::ast::*;
use crate::tokens::*;
use crate::Span;
use chumsky::error::Rich;
use chumsky::input::{Input, MappedInput};
use chumsky::prelude::*;
use chumsky::{extra, select_ref, Parser};
use std::str::FromStr;
pub use error::ParseError;

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
    parser().parse(input(tokens)).into_result().map_err(|errors| errors.into_iter().map(ParseError::new).collect())
}

fn parser<'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, SourceFile, Extra<'tokens>> {
    let mut statement_list_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    let mut format_string_part_parser = Recursive::declare();
    let mut type_parser = Recursive::declare();

    let block = statement_list_parser.clone().delimited_by(
        just(Token::Normal(NormalToken::SigilOpenCurlyBracket)),
        just(Token::Normal(NormalToken::SigilCloseCurlyBracket)),
    );

    let identifier_parser =
        select_ref! { Token::Normal(NormalToken::Identifier(ident)) = e => Identifier {
            span: e.span(),
            name: ident.as_ref().into(),
        } }
        .labelled("identifier");

    let qualified_name = identifier_parser
        .separated_by(just(Token::Normal(NormalToken::SigilDoubleColon)))
        .at_least(1)
        .collect::<Vec<_>>()
        .map_with(|parts, e| {
            QualifiedName {
                span: e.span(),
                parts,
            }
        })
        .labelled("qualified name");

    let single_type =
        select_ref! { Token::Normal(NormalToken::Identifier(ident)) = e => SingleType {
            span: e.span(),
            name: ident.as_ref().into()
        }}
        .labelled("quantity type");

    type_parser.define({
        let single = single_type.map(Type::Single);
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
            });

        let tuple = identifier_parser
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
            });

        single.or(array).or(tuple).labelled("type")
    });

    let literal_parser = {
        let single_value = select_ref! {
            Token::Normal(NormalToken::String(str_tokens)) = e if is_literal_string(str_tokens) => {
                Literal::String(StringLiteral {
                    span: e.span(),
                    content: get_literal_string(str_tokens).expect("non literal string"),
                })
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
        };

        let quantity = select_ref! {
            Token::Normal(NormalToken::LiteralInt(x)) => x,
            Token::Normal(NormalToken::LiteralFloat(x)) => x,
        }
        .then(single_type)
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

        quantity.or(single_value).labelled("literal")
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
        Token::Normal(NormalToken::OperatorOr) => Operator::Or,
        Token::Normal(NormalToken::OperatorXor) => Operator::Xor,
    }
    .labelled("binary operator");

    let unary_operator_parser = select_ref! {
        Token::Normal(NormalToken::OperatorSubtract) => UnaryOperator::Minus,
        Token::Normal(NormalToken::OperatorAdd) => UnaryOperator::Plus,
        Token::Normal(NormalToken::OperatorNot) => UnaryOperator::Not,
    }
    .labelled("unary operator");

    statement_parser.define({
        let expression = expression_parser.clone().map(Statement::Expression);

        let assignment = identifier_parser
            .then(
                just(Token::Normal(NormalToken::SigilColon))
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then_ignore(just(Token::Normal(NormalToken::OperatorAssignment)))
            .then(expression_parser.clone())
            .map_with(|((name, ty), value), e| {
                Statement::Assignment(Assignment {
                    span: e.span(),
                    name,
                    value,
                    ty,
                })
            })
            .labelled("assignment");

        let comment = select_ref! {
            Token::Normal(NormalToken::SingleLineComment(comment) )= e => Comment {
                span: e.span(),
                comment: comment.as_ref().into()
            },
            Token::Normal(NormalToken::MultiLineComment(comment)) = e => Comment {
                span: e.span(),
                comment: comment.as_ref().into()
            }
        }
        .map(Statement::Comment)
        .labelled("comment");

        let arguments = identifier_parser
            .then(
                just(Token::Normal(NormalToken::SigilColon))
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then(
                just(Token::Normal(NormalToken::OperatorAssignment))
                    .ignore_then(literal_parser)
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
            );

        let visibility = select_ref! {
            Token::Normal(NormalToken::KeywordPub) => Visibility::Public,
        }
        .labelled("visibility");

        let module = visibility
            .or_not()
            .then_ignore(just(Token::Normal(NormalToken::KeywordMod)))
            .then(identifier_parser.clone())
            .then(
                block.clone().map(Some)
                    .or(just(Token::Normal(NormalToken::SigilSemiColon)).map(|_| None))
            )
            .map_with(|((visibility, name), body), e| {
                Statement::Module(ModuleDefinition {
                    span: e.span(),
                    attributes: Vec::new(), // todo
                    visibility,
                    name,
                    body,
                })
            });

        let use_parts = identifier_parser
            .map(UseStatementPart::Identifier)
            .or(just(Token::Normal(NormalToken::OperatorMultiply)).map_with(|_, e| UseStatementPart::Glob(e.span())))
            .separated_by(just(Token::Normal(NormalToken::SigilDoubleColon)))
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|parts, e| UseName {
                span: e.span(),
                parts,
            });

        let use_statement = visibility.clone()
            .or_not()
            .then_ignore(just(Token::Normal(NormalToken::KeywordUse)))
            .then(use_parts)
            .then(
                just(Token::Normal(NormalToken::KeywordAs))
                    .ignore_then(identifier_parser.clone())
                    .or_not()
            )
            .map_with(
                |((visibility, name), use_as), e| {
                    Statement::Use(UseStatement {
                        span: e.span(),
                        visibility,
                        name,
                        use_as,
                    })
                },
            );

        let workspace_kind = select_ref! {
            Token::Normal(NormalToken::KeywordSketch) => WorkspaceKind::Sketch,
            Token::Normal(NormalToken::KeywordPart) => WorkspaceKind::Part,
            Token::Normal(NormalToken::KeywordOp) => WorkspaceKind::Op,
        };

        let init = just(Token::Normal(NormalToken::KeywordInit))
            .ignore_then(arguments.clone())
            .then(block.clone())
            .map_with(|(arguments, body), e| {
                Statement::Init(InitDefinition {
                    span: e.span(),
                    arguments,
                    body,
                })
            });

        let workspace = visibility
            .or_not()
            .then(workspace_kind)
            .then(identifier_parser)
            .then(arguments.clone())
            .then(block.clone())
            .map_with(|((((visibility, kind), name), arguments), body), e| {
                Statement::Workbench(WorkbenchDefinition {
                    span: e.span(),
                    kind,
                    attributes: Vec::new(), // todo
                    visibility,
                    name,
                    arguments,
                    body,
                })
            });

        let return_statement = just(Token::Normal(NormalToken::KeywordReturn))
            .ignore_then(expression_parser.clone())
            .map_with(|value, e| {
                Statement::Return(Return {
                    span: e.span(),
                    value,
                })
            });

        let function = visibility
            .or_not()
            .then_ignore(just(Token::Normal(NormalToken::KeywordFn)))
            .then(identifier_parser)
            .then(arguments.clone())
            .then(
                just(Token::Normal(NormalToken::SigilSingleArrow))
                    .ignore_then(type_parser.clone())
                    .or_not(),
            )
            .then(block.clone())
            .map_with(
                |((((visibility, name), arguments), return_type), body), e| {
                    Statement::Function(FunctionDefinition {
                        span: e.span(),
                        visibility,
                        name,
                        arguments,
                        return_type,
                        body,
                    })
                },
            );

        let with_semi = assignment.or(return_statement).or(use_statement)
            .or(expression);

        let without_semi = function.or(init).or(workspace).or(module).or(comment);

        with_semi
            .then_ignore(just(Token::Normal(NormalToken::SigilSemiColon)).labelled("semicolon"))
            .or(without_semi)
            .labelled("statement")
    });

    statement_list_parser.define({
        let trailing_expr = expression_parser.clone().map(Box::new).or_not();
        statement_parser
            .repeated()
            .collect::<Vec<_>>()
            .then(trailing_expr)
            .map_with(|(statements, tail), e| StatementList {
                span: e.span(),
                statements,
                tail,
            })
    });

    expression_parser.define({
        let literal = literal_parser.map(Expression::Literal).labelled("literal");

        let marker = just(Token::Normal(NormalToken::SigilAt))
            .ignore_then(identifier_parser)
            .map(Expression::Marker)
            .labelled("marker");

        let string_format_tokens = select_ref!(
            Token::Normal(NormalToken::String(str_tokens)) if !is_literal_string(str_tokens) => {
                input(str_tokens)
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
            .map(Expression::String);

        let tuple_body = identifier_parser
            .then_ignore(just(Token::Normal(NormalToken::OperatorAssignment)))
            .or_not()
            .then(expression_parser.clone())
            .separated_by(just(Token::Normal(NormalToken::SigilComma)))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenBracket)),
                just(Token::Normal(NormalToken::SigilCloseBracket)),
            );

        let tuple = tuple_body.clone().map_with(|values, e| {
            Expression::Tuple(TupleExpression {
                span: e.span(),
                values,
            })
        });

        let bracketed = expression_parser.clone().delimited_by(
            just(Token::Normal(NormalToken::SigilOpenBracket)),
            just(Token::Normal(NormalToken::SigilCloseBracket)),
        );

        let array_range = expression_parser
            .clone()
            .then_ignore(just(Token::Normal(NormalToken::SigilDoubleDot)))
            .then(expression_parser.clone())
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenSquareBracket)),
                just(Token::Normal(NormalToken::SigilCloseSquareBracket)),
            )
            .map_with(|(start, end), e| {
                Expression::ArrayRange(ArrayRangeExpression {
                    span: e.span(),
                    start: Box::new(start),
                    end: Box::new(end),
                })
            })
            .labelled("array range");

        let array_list = expression_parser
            .clone()
            .separated_by(just(Token::Normal(NormalToken::SigilComma)))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenSquareBracket)),
                just(Token::Normal(NormalToken::SigilCloseSquareBracket)),
            )
            .map_with(|items, e| {
                Expression::ArrayList(ArrayListExpression {
                    span: e.span(),
                    items,
                })
            })
            .labelled("array");

        let call = qualified_name
            .clone()
            .then(tuple_body.map_with(|arguments, e| {
                ArgumentList {
                    span: e.span(),
                    arguments: arguments
                        .into_iter()
                        .map(|(name, value)| match name {
                            Some(name) => Argument::Named(NamedArgument {
                                span: name.span.start..value.span().end,
                                name,
                                value,
                            }),
                            None => Argument::Unnamed(UnnamedArgument {
                                span: value.span(),
                                value,
                            }),
                        })
                        .collect::<Vec<_>>(),
                }
            }))
            .map_with(|(name, arguments), e| {
                Expression::Call(Call {
                    span: e.span(),
                    name,
                    arguments,
                })
            });

        let block_expression = block
            .clone()
            .map(Expression::Block)
            .labelled("block expression");

        let mut if_inner = Recursive::declare();

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
        let if_expression = if_inner.map(Expression::If).labelled("if expression");

        let qualified_name_expr = identifier_parser
            .map_with(|ident, e| QualifiedName {
                span: e.span(),
                parts: vec![ident],
            })
            .foldl_with(
                just(Token::Normal(NormalToken::SigilDoubleColon))
                    .ignore_then(identifier_parser)
                    .repeated(),
                |mut acc, part, _| {
                    acc.span.end = part.span.end;
                    acc.parts.push(part);
                    acc
                },
            )
            .map(Expression::QualifiedName);

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
            .or(if_expression);

        let binary_expression = base.clone().foldl_with(
            binary_operator_parser.then(base.clone()).repeated(),
            |lhs, (op, rhs), e| {
                Expression::BinaryOperation(BinaryOperation {
                    span: e.span(),
                    lhs: lhs.into(),
                    operation: op,
                    rhs: rhs.into(),
                })
            },
        );

        let unary_expression = unary_operator_parser.then(base).map_with(|(op, rhs), e| {
            Expression::UnaryOperation(UnaryOperation {
                span: e.span(),
                operation: op,
                rhs: rhs.into(),
            })
        });

        unary_expression
            .or(binary_expression)
            .labelled("expression")
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
            });

        content.or(format_expr)
    });

    statement_list_parser.map_with(move |statements, ex| SourceFile {
        span: ex.span(),
        statements,
    })
}
