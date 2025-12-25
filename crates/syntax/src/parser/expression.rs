use crate::Span;
use crate::ast::{ArrayListExpression, ArrayRangeExpression, BinaryOperation, Expression, Identifier, IntegerLiteral, Literal, Operator, StatementList};
use crate::parser::literal::literal_parser;
use crate::parser::statement::statement_list_parser;
use crate::tokens::Token;
use chumsky::error::Rich;
use chumsky::input::BorrowInput;
use chumsky::prelude::*;
use chumsky::{Parser, extra, select_ref};

pub fn identifier_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Identifier, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    select_ref! { Token::Identifier(ident) = e => Identifier {
        span: e.span(),
        name: (*ident).into()
    } }
    .labelled("identifier")
}

pub fn expression_parser<'tokens, 'src: 'tokens, I>(
    statement_list_parser: impl Parser<'tokens, I, StatementList, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone + 'tokens
)
-> impl Parser<'tokens, I, Expression, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    recursive(|exp_parser| {
        let literal = literal_parser()
            .map(Expression::Literal)
            .labelled("literal");
        let ident = identifier_parser().map(Expression::Identifier);

        let bracketed = exp_parser.clone().delimited_by(
            just(Token::SigilOpenBracket),
            just(Token::SigilCloseBracket),
        );

        let array_range = exp_parser
            .clone()
            .then_ignore(just(Token::SigilDoubleDot))
            .then(exp_parser.clone())
            .delimited_by(
                just(Token::SigilOpenSquareBracket),
                just(Token::SigilCloseSquareBracket),
            )
            .map_with(|(start, end), e| {
                Expression::ArrayRange(ArrayRangeExpression {
                    span: e.span(),
                    start: Box::new(start),
                    end: Box::new(end),
                })
            })
            .labelled("array range");

        let array_list = exp_parser
            .clone()
            .separated_by(just(Token::SigilComma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(Token::SigilOpenSquareBracket),
                just(Token::SigilCloseSquareBracket),
            )
            .map_with(|items, e| {
                Expression::ArrayList(ArrayListExpression {
                    span: e.span(),
                    items,
                })
            })
            .labelled("array");

        let block = statement_list_parser
            .delimited_by(
                just(Token::SigilOpenCurlyBracket),
                just(Token::SigilCloseCurlyBracket),
            )
            .map(Expression::Block)
            .labelled("block expression");

        let base = literal
            .or(ident)
            .or(bracketed)
            .or(array_range)
            .or(array_list)
            .or(block);

        let binary_expression = base.clone().foldl_with(
            binary_operator_parser().then(base).repeated(),
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
    })
}

pub fn binary_operator_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Operator, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    select_ref! {
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
        Token::OperatorOr => Operator::Or,
        Token::OperatorXor => Operator::Xor,
    }
    .labelled("binary operator")
}
