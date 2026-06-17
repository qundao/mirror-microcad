// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    impl_tokens,
    semantic_tokens::{SemanticTokens, TokenContext},
};

use super::{TokenModifier, TokenType};

use microcad_driver::prelude as mu;
use mu::ast;

impl_tokens!(ast::LocalAssignment => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.attributes
            .iter()
            .for_each(|attr| attr.semantic_tokens(ctx));
    ctx.push_token(&self_.name.span, TokenType::VARIABLE, &[]);
    if let Some(ty) = self_.ty.as_ref() { ty.semantic_tokens(ctx) }
});

impl_tokens!(ast::AttributeCommand => |self_, ctx| {
    match &self_ {
        ast::AttributeCommand::Ident(id) => ctx.push_token(&id.span, TokenType::PROPERTY, &[]),
        ast::AttributeCommand::Assignment(local_assignment) => local_assignment.semantic_tokens(ctx),
        ast::AttributeCommand::Call(call) => call.semantic_tokens(ctx),
    }
});

impl_tokens!(ast::Attribute => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.commands.iter().for_each(|command| command.semantic_tokens(ctx));
});

impl_tokens!(ast::DocBlock => |self_, ctx| {
    ctx.push_token(&self_.span, TokenType::COMMENT, &[TokenModifier::DOCUMENTATION]);
});

impl_tokens!(ast::Parameter => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    if let Some(expr) = self_.default.as_ref() { expr.semantic_tokens(ctx) }
    ctx.push_token(&self_.name.span, TokenType::PARAMETER, &[]);
    if let Some(ty) = self_.ty.as_ref() { ty.semantic_tokens(ctx) }
});

impl_tokens!(ast::ParameterList => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.parameters.iter().for_each(|param| param.semantic_tokens(ctx));
});

impl_tokens!(ast::InitDefinition => |self_, ctx| {
        self_.extras.semantic_tokens(ctx);
        self_.doc.semantic_tokens(ctx);
        self_.attributes
            .iter()
            .for_each(|attr| attr.semantic_tokens(ctx));
        ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
        self_.parameters.semantic_tokens(ctx);
        self_.body.semantic_tokens(ctx);
});

impl_tokens!(ast::Body => statements);
//impl_tokens!(ast::Visibility => TokenType::KEYWORD);

impl_tokens!(ast::WorkbenchDefinition => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.doc.semantic_tokens(ctx);
    self_.attributes.iter().for_each(|attr| attr.semantic_tokens(ctx));
    // TODO: self_.visibility.as_ref().map(|vis| vis.semantic_tokens(ctx));
    ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
    ctx.push_token(&self_.name.span, TokenType::FUNCTION, &[]);
    self_.parameters.semantic_tokens(ctx);
    self_.body.semantic_tokens(ctx);
});

impl_tokens!(ast::Return => |self_, ctx| {
    ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
    if let Some(expr) = self_.value.as_ref() { expr.semantic_tokens(ctx) }
});

impl_tokens!(ast::InlineModule => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.doc.semantic_tokens(ctx);
    self_.attributes.iter().for_each(|attr| attr.semantic_tokens(ctx));
    // TODO: self_.visibility.as_ref().map(|vis| vis.semantic_tokens(ctx));
    ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
    ctx.push_token(&self_.name.span, TokenType::NAMESPACE, &[]);
    self_.body.semantic_tokens(ctx);
});

impl_tokens!(ast::FileModule => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.doc.semantic_tokens(ctx);
    self_.attributes.iter().for_each(|attr| attr.semantic_tokens(ctx));
    // TODO: self_.visibility.as_ref().map(|vis| vis.semantic_tokens(ctx));
    ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
    ctx.push_token(&self_.name.span, TokenType::NAMESPACE, &[]);
});

impl_tokens!(ast::FunctionDefinition => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.doc.semantic_tokens(ctx);
    self_.attributes.iter().for_each(|attr| attr.semantic_tokens(ctx));
    // TODO: self_.visibility.as_ref().map(|vis| vis.semantic_tokens(ctx));
    ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
    ctx.push_token(&self_.name.span, TokenType::FUNCTION, &[]);
    self_.parameters.semantic_tokens(ctx);
    self_.body.semantic_tokens(ctx);
});

impl_tokens!(ast::UseStatementPart => |self_, ctx| {
    match &self_ {
        ast::UseStatementPart::Identifier(id) => ctx.push_token(&id.span, TokenType::NAMESPACE, &[]),
        ast::UseStatementPart::Glob(span) => ctx.push_token(span, TokenType::OPERATOR, &[]),
        _ => {}
    }
});

impl_tokens!(ast::UseName => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.parts.iter().for_each(|part| part.semantic_tokens(ctx));
});

impl_tokens!(ast::UseStatement => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.attributes.iter().for_each(|attr| attr.semantic_tokens(ctx));
    // TODO: self_.visibility.as_ref().map(|vis| vis.semantic_tokens(ctx));
    ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
    self_.name.semantic_tokens(ctx);
    if let Some(as_) = self_.use_as.as_ref() { ctx.push_token(&as_.span, TokenType::NAMESPACE, &[]) }
});

impl_tokens!(ast::ConstAssignment => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.doc.semantic_tokens(ctx);
    self_.attributes.iter().for_each(|attr| attr.semantic_tokens(ctx));
    // TODO: self_.visibility.as_ref().map(|vis| vis.semantic_tokens(ctx));
    ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
    ctx.push_token(&self_.name.span, TokenType::PROPERTY, &[]);
    if let Some(ty) = self_.ty.as_ref() { ty.semantic_tokens(ctx) }
    self_.value.semantic_tokens(ctx);
});

impl_tokens!(ast::PropertyAssignment => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.attributes.iter().for_each(|attr| attr.semantic_tokens(ctx));
    ctx.push_token(&self_.keyword_span, TokenType::KEYWORD, &[]);
    ctx.push_token(&self_.name.span, TokenType::PROPERTY, &[]);
    if let Some(ty) = self_.ty.as_ref() { ty.semantic_tokens(ctx) }
    self_.value.semantic_tokens(ctx);
});

impl_tokens!(ast::InnerDocComment => |self_, ctx| {
    ctx.push_token(&self_.span, TokenType::COMMENT, &[TokenModifier::DOCUMENTATION]);
});

impl_tokens!(ast::ExpressionStatement => |self_, ctx | {
    self_.extras.semantic_tokens(ctx);
    self_.attributes.iter().for_each(|attr| attr.semantic_tokens(ctx));
    self_.expression.semantic_tokens(ctx);
});

impl_tokens!(ast::Statement => |self_, ctx| {
    match &self_ {
        ast::Statement::Workbench(workbench_definition) => workbench_definition.semantic_tokens(ctx),
        ast::Statement::InlineModule(inline_module) => inline_module.semantic_tokens(ctx),
        ast::Statement::FileModule(file_module) => file_module.semantic_tokens(ctx),
        ast::Statement::Function(function_definition) => function_definition.semantic_tokens(ctx),
        ast::Statement::Use(use_statement) => use_statement.semantic_tokens(ctx),
        ast::Statement::Const(const_assignment) => const_assignment.semantic_tokens(ctx),
        ast::Statement::Init(init_definition) => init_definition.semantic_tokens(ctx),
        ast::Statement::Return(ret) => ret.semantic_tokens(ctx),
        ast::Statement::InnerAttribute(attribute) => attribute.semantic_tokens(ctx),
        ast::Statement::InnerDocComment(inner_doc_comment) => inner_doc_comment.semantic_tokens(ctx),
        ast::Statement::LocalAssignment(local_assignment) => local_assignment.semantic_tokens(ctx),
        ast::Statement::Property(property_assignment) => property_assignment.semantic_tokens(ctx),
        ast::Statement::Expression(expression_statement) => expression_statement.semantic_tokens(ctx),
        ast::Statement::Error(_) => {}
    }
});

impl_tokens!(ast::StatementList => |self_, ctx| {
    self_.extras.semantic_tokens(ctx);
    self_.statements.iter().for_each(|(stmt, extras)| {
        stmt.semantic_tokens(ctx);
        extras.semantic_tokens(ctx);
    });
    if let Some(expr) = self_.tail.as_ref() { expr.semantic_tokens(ctx) }
});
