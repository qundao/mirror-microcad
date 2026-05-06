// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};
use microcad_syntax::ast;

/// Note: These constructors are a workaround until the assignment in microcad-lang is split up
impl ir::Assignment {
    fn from_ast_local(
        node: &ast::LocalAssignment,
        context: &LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(ir::Assignment {
            doc: ir::DocBlock::default(),
            visibility: ir::Visibility::Private,
            id: ir::Identifier::from_ast(&node.name, context)?,
            qualifier: ir::Qualifier::Value,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| ir::TypeAnnotation::from_ast(ty, context))
                .transpose()?,
            expression: ir::Expression::from_ast(&node.value, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }

    fn from_ast_prop(
        node: &ast::PropertyAssignment,
        context: &LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(ir::Assignment {
            doc: ir::DocBlock::from_ast(&node.doc, context)?,
            visibility: ir::Visibility::Private,
            id: ir::Identifier::from_ast(&node.name, context)?,
            qualifier: ir::Qualifier::Prop,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| ir::TypeAnnotation::from_ast(ty, context))
                .transpose()?,
            expression: ir::Expression::from_ast(&node.value, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }

    fn from_ast_const(
        node: &ast::ConstAssignment,
        context: &LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(ir::Assignment {
            doc: ir::DocBlock::from_ast(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|v| ir::Visibility::from_ast(v, context))
                .transpose()?
                .unwrap_or_default(),
            id: ir::Identifier::from_ast(&node.name, context)?,
            qualifier: ir::Qualifier::Const,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| ir::TypeAnnotation::from_ast(ty, context))
                .transpose()?,
            expression: ir::Expression::from_ast(&node.value, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

/// Note: These constructors are a workaround until the assignment in microcad-lang is split up
impl ir::AssignmentStatement {
    fn from_ast_local(
        node: &ast::LocalAssignment,
        context: &LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(Self {
            attribute_list: ir::AttributeList::from_ast(&node.attributes, context)?,
            assignment: std::rc::Rc::new(ir::Assignment::from_ast_local(node, context)?),
            src_ref: context.src_ref(&node.span),
        })
    }

    fn from_ast_prop(
        node: &ast::PropertyAssignment,
        context: &LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(Self {
            attribute_list: ir::AttributeList::from_ast(&node.attributes, context)?,
            assignment: std::rc::Rc::new(ir::Assignment::from_ast_prop(node, context)?),
            src_ref: context.src_ref(&node.span),
        })
    }

    fn from_ast_const(
        node: &ast::ConstAssignment,
        context: &LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(Self {
            attribute_list: ir::AttributeList::from_ast(&node.attributes, context)?,
            assignment: std::rc::Rc::new(ir::Assignment::from_ast_const(node, context)?),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for ir::IfStatement {
    type AstNode = ast::If;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::IfStatement {
            if_ref: context.src_ref(&node.if_span),
            cond: ir::Expression::from_ast(&node.condition, context)?,
            body: ir::Body::from_ast(&node.body, context)?,
            next_if_ref: node.next_if_span.as_ref().map(|span| context.src_ref(span)),
            next_if: node
                .next_if
                .as_ref()
                .map(|next| ir::IfStatement::from_ast(next, context))
                .transpose()?
                .map(Box::new),
            else_ref: node.else_span.as_ref().map(|span| context.src_ref(span)),
            body_else: node
                .else_body
                .as_ref()
                .map(|body| ir::Body::from_ast(body, context))
                .transpose()?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for ir::ExpressionStatement {
    type AstNode = ast::ExpressionStatement;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::ExpressionStatement {
            src_ref: context.src_ref(&node.span),
            attribute_list: ir::AttributeList::from_ast(&node.attributes, context)?,
            expression: ir::Expression::from_ast(&node.expression, context)?,
        })
    }
}

impl FromAst for ir::Statement {
    type AstNode = ast::Statement;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(match node {
            ast::Statement::InlineModule(module) => ir::Statement::Module(std::rc::Rc::new(
                ir::ModuleDefinition::from_ast_inline(module, context)?,
            )),
            ast::Statement::FileModule(module) => ir::Statement::Module(std::rc::Rc::new(
                ir::ModuleDefinition::from_ast_file(module, context)?,
            )),
            ast::Statement::Use(statement) => {
                ir::Statement::Use(ir::UseStatement::from_ast(statement, context)?)
            }
            ast::Statement::Expression(ast::ExpressionStatement {
                expression: ast::Expression::If(if_statement),
                ..
            }) => ir::Statement::If(ir::IfStatement::from_ast(if_statement, context)?),
            ast::Statement::Expression(statement) => {
                ir::Statement::Expression(ir::ExpressionStatement::from_ast(statement, context)?)
            }
            ast::Statement::Workbench(w) => {
                ir::Statement::Workbench(<std::rc::Rc<ir::WorkbenchDefinition>>::from_ast(
                    w, context,
                )?)
            }
            ast::Statement::Function(f) => ir::Statement::Function(std::rc::Rc::new(
                ir::FunctionDefinition::from_ast(f, context)?,
            )),
            ast::Statement::Init(i) => {
                ir::Statement::Init(std::rc::Rc::new(ir::InitDefinition::from_ast(i, context)?))
            }
            ast::Statement::Return(r) => {
                ir::Statement::Return(ir::ReturnStatement::from_ast(r, context)?)
            }
            ast::Statement::InnerAttribute(a) => {
                ir::Statement::InnerAttribute(ir::Attribute::from_ast(a, context)?)
            }
            ast::Statement::InnerDocComment(i) => {
                ir::Statement::InnerDocComment(ir::InnerDocComment::from_ast(i, context)?)
            }
            ast::Statement::LocalAssignment(a) => {
                ir::Statement::Assignment(ir::AssignmentStatement::from_ast_local(a, context)?)
            }
            ast::Statement::Property(a) => {
                ir::Statement::Assignment(ir::AssignmentStatement::from_ast_prop(a, context)?)
            }
            ast::Statement::Const(a) => {
                ir::Statement::Assignment(ir::AssignmentStatement::from_ast_const(a, context)?)
            }
            ast::Statement::Error(span) => {
                return Err(LowerError::InvalidStatement {
                    src_ref: context.src_ref(span),
                });
            }
        })
    }
}

impl FromAst for ir::ReturnStatement {
    type AstNode = ast::Return;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::ReturnStatement {
            keyword_ref: context.src_ref(&node.keyword_span),
            result: node
                .value
                .as_ref()
                .map(|res| ir::Expression::from_ast(res, context))
                .transpose()?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for ir::StatementList {
    type AstNode = ast::StatementList;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        let mut statements = Vec::new();
        node.statements.iter().try_for_each(|(statement, _)| {
            statements.push(ir::Statement::from_ast(statement, context)?);
            Ok(())
        })?;

        if let Some(tail) = &node.tail {
            statements.push(ir::Statement::Expression(
                ir::ExpressionStatement::from_ast(tail, context)?,
            ));
        }

        Ok(ir::StatementList(statements))
    }
}
