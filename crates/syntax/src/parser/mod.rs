use crate::Span;
use crate::ast::*;
use crate::tokens::*;
use chumsky::error::Rich;
use chumsky::input::BorrowInput;
use chumsky::prelude::*;
use chumsky::{Parser, extra, select_ref};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

type Extra<'tokens, 'src> = extra::Err<Rich<'tokens, Token<'src>, Span>>;

pub fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, SourceFile, Extra<'tokens, 'src>> + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    let mut statement_list_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    // let mut format_string_parser = Recursive::declare();

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
            });

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
        let literal = literal_parser
            .map(Expression::Literal)
            .labelled("literal");
        let ident = identifier_parser.map(Expression::Identifier);

        // let string_format = select_ref!(
        //         Token::Normal(NormalToken::String(str_tokens)) = e if !is_literal_string(str_tokens) => {
        //             FormatString {
        //                 span: e.span(),
        //                 parts: todo!(),
        //             }
        //         },
        //     ).map(Expression::String);

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
            // .or(string_format)
            .or(ident)
            .or(bracketed)
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

    // format_string_parser.define({
    //     let content = select_ref!(
    //         Token::String(StringToken::Content(str)) = e => {
    //             StringPart::Content(StringContent {
    //                 span: e.span(),
    //                 content: (*str).into(),
    //             })
    //         },
    //         Token::String(StringToken::Escaped(str)) => {
    //             StringPart::Char(str.chars().nth(2).expect("invalid escaped token"))
    //         },
    //         Token::String(StringToken::EscapedCurlyOpen ) => {
    //             StringPart::Char('{')
    //         },
    //         Token::String(StringToken::EscapedCurlyClose) => {
    //             StringPart::Char('}')
    //         },
    //         Token::String(StringToken::BackSlash) => {
    //             StringPart::Char('\\')
    //         },
    //     );
    //
    //     let format_tokens = select_ref!(
    //         Token::String(StringToken::FormatStart(args)) => args
    //             .as_slice()
    //             .map(1..1, |spanned: &SpannedToken<Token>| {
    //                 (&spanned.token, &spanned.span)
    //             })
    //     );
    //     let format_expr = expression_parser.nested_in(format_tokens);
    //     let format_expr = format_expr.map_with(|expr, e| {
    //         StringPart::Expression(StringExpression {
    //             span: e.span(),
    //             expression: expr,
    //             accuracy: None,
    //             width: None,
    //         })
    //     });
    //
    //     let part = content.or(format_expr);
    //
    //     part.repeated()
    //         .collect::<Vec<_>>()
    //         .map_with(|parts, e| FormatString {
    //             span: e.span(),
    //             parts,
    //         })
    //         .labelled("format string")
    // });

    statement_list_parser.map_with(|statements, ex| SourceFile {
        span: ex.span(),
        statements,
    })
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFloat(ParseFloatError),
    InvalidInt(ParseIntError),
    Unknown,
}
