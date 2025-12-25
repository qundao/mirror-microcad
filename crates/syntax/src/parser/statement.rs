use crate::Span;
use crate::ast::{Assignment, Comment, Statement, StatementList};
use crate::parser::expression::{expression_parser, identifier_parser};
use crate::tokens::Token;
use chumsky::error::Rich;
use chumsky::input::BorrowInput;
use chumsky::prelude::just;
use chumsky::prelude::*;
use chumsky::{IterParser, Parser, extra};

pub fn statement_parser<'tokens, 'src: 'tokens, I>(
    statement_list_parser: impl Parser<'tokens, I, StatementList, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone + 'tokens
)
-> impl Parser<'tokens, I, Statement, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    let expression = expression_parser(statement_list_parser.clone()).map(Statement::Expression);

    let assigment = identifier_parser()
        .then_ignore(just(Token::OperatorAssignment))
        .then(expression_parser(statement_list_parser))
        .map_with(|(name, value), e| {
            Statement::Assignment(Assignment {
                span: e.span(),
                name,
                value,
                ty: None, // todo
            })
        });

    let comment = select_ref! {
        Token::SingleLineComment(comment) = e => Comment {
            span: e.span(),
            comment: (*comment).into()
        },
        Token::MultiLineComment(comment) = e => Comment {
            span: e.span(),
            comment: (*comment).into()
        }
    }
    .map(Statement::Comment)
    .labelled("comment");

    let statement = assigment.or(expression);

    statement
        .then_ignore(just(Token::SigilSemiColon).labelled("semicolon"))
        .or(comment)
        .labelled("statement")
}

pub fn statement_list_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, StatementList, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    recursive(|statement_list_parser| {
        let trailing_expr = expression_parser(statement_list_parser.clone()).map(Box::new).or_not();
        let with_tail = statement_parser(statement_list_parser)
            .repeated()
            .collect::<Vec<_>>()
            .then(trailing_expr)
            .map_with(|(statements, tail), e| StatementList {
                span: e.span(),
                statements,
                tail,
            });

        with_tail
    })
}
