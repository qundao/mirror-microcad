// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_driver::prelude as mu;
use mu::ast;

use super::{SemanticTokens, TokenContext, TokenType};

use crate::impl_tokens;

impl_tokens!(ast::StringLiteral => TokenType::STRING);
impl_tokens!(ast::BoolLiteral => TokenType::KEYWORD);
impl_tokens!(ast::IntegerLiteral => TokenType::NUMBER);
impl_tokens!(ast::FloatLiteral => TokenType::NUMBER);
impl_tokens!(ast::QuantityLiteral => TokenType::NUMBER);

impl_tokens!(ast::Literal => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    match &self_.literal {
        ast::LiteralKind::String(s) => s.semantic_tokens(ctx),
        ast::LiteralKind::Bool(b) => b.semantic_tokens(ctx),
        ast::LiteralKind::Integer(i) => i.semantic_tokens(ctx),
        ast::LiteralKind::Float(f) => f.semantic_tokens(ctx),
        ast::LiteralKind::Quantity(q) => q.semantic_tokens(ctx),
        ast::LiteralKind::Error(_) => {}
    }
});

impl_tokens!(ast::TupleItem => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    if let Some(id) = self_.id.as_ref() { ctx.push_token(&id.span, TokenType::PROPERTY, &[]) };
    self_.expr.semantic_tokens(ctx);
});

impl_tokens!(ast::TupleExpression => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.values
        .iter()
        .for_each(|item| item.semantic_tokens(ctx));
});

impl_tokens!(ast::ArrayItem => extras, expr);

impl_tokens!(ast::ArrayRangeExpression => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.start.semantic_tokens(ctx);
    self_.end.semantic_tokens(ctx);
    if let Some(unit) = self_.unit.as_ref() { unit.semantic_tokens(ctx) }
});

impl_tokens!(ast::ArrayListExpression => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.items
        .iter()
        .for_each(|item| item.semantic_tokens(ctx));
    if let Some(unit) = self_.unit.as_ref() { unit.semantic_tokens(ctx) }
});

impl_tokens!(ast::StringCharacter => TokenType::STRING);
impl_tokens!(ast::StringExpression => extras, expr);

impl_tokens!(ast::StringPart => [Char, Content, Expression]);

impl_tokens!(ast::FormatString => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.parts
         .iter()
         .for_each(|part| part.semantic_tokens(ctx));
});

impl<'ast> SemanticTokens<'ast> for ast::QualifiedName {
    fn semantic_tokens(&'ast self, context: &'ast mut TokenContext) {
        // TODO The token type might be different than `NAMESPACE`, depending on the context.
        self.extras.semantic_tokens(context);
        self.parts
            .iter()
            .for_each(|part| context.push_token(&part.span, TokenType::NAMESPACE, &[]));
    }
}

impl_tokens!(ast::BinaryOperator => TokenType::OPERATOR);
impl_tokens!(ast::BinaryOperation => lhs, operation, rhs);
impl_tokens!(ast::UnaryOperator => TokenType::OPERATOR);
impl_tokens!(ast::UnaryOperation => extras, operation, rhs);

impl_tokens!(ast::NamedArgument => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    ctx.push_token(&self_.id.span, TokenType::PARAMETER, &[]);
    self_.expr.semantic_tokens(ctx);
});

impl_tokens!(ast::UnnamedArgument => extras, expr);
impl_tokens!(ast::Argument => [Unnamed, Named]);

impl_tokens!(ast::ArgumentList => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.arguments
        .iter()
        .for_each(|arg| arg.semantic_tokens(ctx));
});

impl_tokens!(ast::Call => extras, name, arguments);

impl_tokens!(ast::Element => extras, inner);

impl_tokens!(ast::ElementInner => |self_, ctx| {
    match &self_ {
        ast::ElementInner::Attribute(identifier) => {
            ctx.push_token(&identifier.span, TokenType::DECORATOR, &[])
        }
        ast::ElementInner::Tuple(identifier) => {
            ctx.push_token(&identifier.span, TokenType::PROPERTY, &[])
        }
        ast::ElementInner::Method(call) => call.semantic_tokens(ctx),
        ast::ElementInner::ArrayElement(expression) => expression.semantic_tokens(ctx),
    }
});

impl_tokens!(ast::ElementAccess => |self_, ctx| {
    self_.expr.semantic_tokens(ctx);
    self_.element_chain.iter().for_each(|element| element.semantic_tokens(ctx));
});

impl_tokens!(ast::If => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    ctx.push_token(&self_.if_span, TokenType::KEYWORD, &[]);
    self_.condition.semantic_tokens(ctx);
    if let Some(span) = self_.else_span.as_ref() { ctx.push_token(span, TokenType::KEYWORD, &[]) }
    if let Some(body) = self_.else_body.as_ref() { body.semantic_tokens(ctx) }
    if let Some(if_) = self_.next_if.as_ref() { if_.semantic_tokens(ctx) }
});

impl_tokens!(ast::Expression => |self_, ctx| {
    match &self_ {
        ast::Expression::Literal(literal) => literal.semantic_tokens(ctx),
        ast::Expression::Bracketed(expression, _) => expression.semantic_tokens(ctx),
        ast::Expression::Tuple(tuple_expression) => tuple_expression.semantic_tokens(ctx),
        ast::Expression::ArrayRange(array_range_expression) => {
            array_range_expression.semantic_tokens(ctx)
        }
        ast::Expression::ArrayList(array_list_expression) => {
            array_list_expression.semantic_tokens(ctx)
        }
        ast::Expression::String(format_string) => format_string.semantic_tokens(ctx),
        ast::Expression::QualifiedName(qualified_name) => qualified_name.semantic_tokens(ctx),
        ast::Expression::Marker(identifier) => {
            ctx.push_token(&identifier.span, TokenType::EVENT, &[])
        }
        ast::Expression::BinaryOperation(binary_operation) => {
            binary_operation.semantic_tokens(ctx)
        }
        ast::Expression::UnaryOperation(unary_operation) => {
            unary_operation.semantic_tokens(ctx)
        }
        ast::Expression::Body(body) => body.semantic_tokens(ctx),
        ast::Expression::Call(call) => call.semantic_tokens(ctx),
        ast::Expression::ElementAccess(element_access) => element_access.semantic_tokens(ctx),
        ast::Expression::If(if_) => if_.semantic_tokens(ctx),
        ast::Expression::Error(_) => {}
    }
});
