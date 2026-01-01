use crate::Span;
use crate::ast::*;
use crate::tokens::*;
use chumsky::error::Rich;
use chumsky::input::{Input, MappedInput};
use chumsky::prelude::*;
use chumsky::{Parser, extra, select_ref};
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
) -> Result<SourceFile, Vec<Rich<'tokens, Token<'tokens>, std::ops::Range<usize>>>> {
    parser().parse(input(tokens)).into_result()
}

fn parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, SourceFile, Extra<'tokens>> {
    let mut statement_list_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    let mut format_string_part_parser = Recursive::declare();

    let identifier_parser =
        select_ref! { Token::Normal(NormalToken::Identifier(ident)) = e => Identifier {
            span: e.span(),
            name: (*ident).into()
        } }
        .labelled("identifier");

    let literal_parser = {
        let single_value = select_ref! {
            Token::Normal(NormalToken::String(str_tokens)) = e if is_literal_string(str_tokens) => {
                Literal::String(StringContent {
                    span: e.span(),
                    content: get_literal_string(str_tokens).expect("non literal string"),
                })
            },
            Token::Normal(NormalToken::LiteralFloat(x)) = e => {
                match f64::from_str(x) {
                    Ok(value) => Literal::Quantity(QuantityLiteral {
                    value,
                    span: e.span(),
                    ty: None,
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

        single_value.labelled("literal")
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

    statement_parser.define({
        let expression = expression_parser.clone().map(Statement::Expression);

        let assigment = identifier_parser
            .then_ignore(just(Token::Normal(NormalToken::OperatorAssignment)))
            .then(expression_parser.clone())
            .map_with(|(name, value), e| {
                Statement::Assignment(Assignment {
                    span: e.span(),
                    name,
                    value,
                    ty: None, // todo
                })
            })
            .labelled("assignment");

        let comment = select_ref! {
            Token::Normal(NormalToken::SingleLineComment(comment) )= e => Comment {
                span: e.span(),
                comment: (*comment).into()
            },
            Token::Normal(NormalToken::MultiLineComment(comment)) = e => Comment {
                span: e.span(),
                comment: (*comment).into()
            }
        }
        .map(Statement::Comment)
        .labelled("comment");

        let statement = assigment.or(expression);

        statement
            .then_ignore(just(Token::Normal(NormalToken::SigilSemiColon)).labelled("semicolon"))
            .or(comment)
            .labelled("statement")
    });

    statement_list_parser.define({
        let trailing_expr = expression_parser.clone().map(Box::new).or_not();
        let with_tail = statement_parser
            .repeated()
            .collect::<Vec<_>>()
            .then(trailing_expr)
            .map_with(|(statements, tail), e| StatementList {
                span: e.span(),
                statements,
                tail,
            });

        with_tail
    });

    expression_parser.define({
        let literal = literal_parser.map(Expression::Literal).labelled("literal");
        let ident = identifier_parser.map(Expression::Identifier);

        let qualified_name = identifier_parser
            .separated_by(just(Token::Normal(NormalToken::SigilDoubleColon)))
            .at_least(2)
            .collect::<Vec<_>>()
            .map_with(|parts, e| Expression::QualifiedName(QualifiedName {
                span: e.span(),
                parts
            }));

        let string_format_tokens = select_ref!(
            Token::Normal(NormalToken::String(str_tokens)) if !is_literal_string(str_tokens) => {
                input(&str_tokens)
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

        let tuple = expression_parser
            .clone()
            .separated_by(just(Token::Normal(NormalToken::SigilComma)))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenBracket)),
                just(Token::Normal(NormalToken::SigilCloseBracket)),
            )
            .map_with(|values, e| {
                Expression::Tuple(TupleExpression {
                    span: e.span(),
                    values,
                })
            });

        let named_tuple = identifier_parser
            .then_ignore(just(Token::Normal(NormalToken::OperatorAssignment)))
            .then(expression_parser.clone())
            .separated_by(just(Token::Normal(NormalToken::SigilComma)))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenBracket)),
                just(Token::Normal(NormalToken::SigilCloseBracket)),
            )
            .map_with(|values, e| {
                Expression::NamedTuple(NamedTupleExpression {
                    span: e.span(),
                    values: values.into_iter().collect(),
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

        let block = statement_list_parser
            .clone()
            .delimited_by(
                just(Token::Normal(NormalToken::SigilOpenCurlyBracket)),
                just(Token::Normal(NormalToken::SigilCloseCurlyBracket)),
            )
            .map(Expression::Block)
            .labelled("block expression");

        let base = literal
            .or(string_format)
            .or(qualified_name)
            .or(ident)
            .or(bracketed)
            .or(tuple)
            .or(named_tuple)
            .or(array_range)
            .or(array_list)
            .or(block);

        let binary_expression = base.clone().foldl_with(
            binary_operator_parser.then(base).repeated(),
            |lhs, (op, rhs), e| {
                Expression::BinaryOperation(BinaryOperation {
                    span: e.span(),
                    lhs: lhs.into(),
                    operation: op,
                    rhs: rhs.into(),
                })
            },
        );

        binary_expression.labelled("expression")
    });

    format_string_part_parser.define({
        let content = select_ref!(
            Token::String(StringToken::Content(str)) = e => {
                StringPart::Content(StringContent {
                    span: e.span(),
                    content: (*str).into(),
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
                input(&args)
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
