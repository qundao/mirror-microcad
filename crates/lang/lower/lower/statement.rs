// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};
use microcad_lang_parse::ast;

/// Note: These constructors are a workaround until the assignment in microcad-lang is split up
impl ir::Assignment {
    fn from_ast_local(
        node: &ast::LocalAssignment,
        context: &mut LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(ir::Assignment {
            doc: ir::DocBlock::default(),
            visibility: ir::Visibility::Private,
            id: ir::Identifier::lower(&node.id, context)?,
            qualifier: ir::Qualifier::Value,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| ir::TypeAnnotation::lower(ty, context))
                .transpose()?,
            expression: ir::Expression::lower(&node.expr, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }

    fn from_ast_prop(
        node: &ast::PropertyAssignment,
        context: &mut LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(ir::Assignment {
            doc: ir::DocBlock::lower(&node.doc, context)?,
            visibility: ir::Visibility::Private,
            id: ir::Identifier::lower(&node.id, context)?,
            qualifier: ir::Qualifier::Prop,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| ir::TypeAnnotation::lower(ty, context))
                .transpose()?,
            expression: ir::Expression::lower(&node.value, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }

    fn from_ast_const(
        node: &ast::ConstAssignment,
        context: &mut LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(ir::Assignment {
            doc: ir::DocBlock::lower(&node.doc, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|v| ir::Visibility::lower(v, context))
                .transpose()?
                .unwrap_or_default(),
            id: ir::Identifier::lower(&node.id, context)?,
            qualifier: ir::Qualifier::Const,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| ir::TypeAnnotation::lower(ty, context))
                .transpose()?,
            expression: ir::Expression::lower(&node.expr, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

/// Note: These constructors are a workaround until the assignment in microcad-lang is split up
impl ir::AssignmentStatement {
    fn from_ast_local(
        node: &ast::LocalAssignment,
        context: &mut LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(Self {
            attribute_list: ir::AttributeList::lower(&node.attributes, context)?,
            assignment: std::rc::Rc::new(ir::Assignment::from_ast_local(node, context)?),
            src_ref: context.src_ref(&node.span),
        })
    }

    fn from_ast_prop(
        node: &ast::PropertyAssignment,
        context: &mut LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(Self {
            attribute_list: ir::AttributeList::lower(&node.attributes, context)?,
            assignment: std::rc::Rc::new(ir::Assignment::from_ast_prop(node, context)?),
            src_ref: context.src_ref(&node.span),
        })
    }

    fn from_ast_const(
        node: &ast::ConstAssignment,
        context: &mut LowerContext,
    ) -> Result<Self, LowerError> {
        Ok(Self {
            attribute_list: ir::AttributeList::lower(&node.attributes, context)?,
            assignment: std::rc::Rc::new(ir::Assignment::from_ast_const(node, context)?),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower for ir::If {
    type AstNode = ast::If;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::If {
            if_ref: context.src_ref(&node.if_span),
            cond: ir::Expression::lower(&node.condition, context)?,
            body: ir::Body::lower(&node.body, context)?,
            next_if_ref: node.next_if_span.as_ref().map(|span| context.src_ref(span)),
            next_if: node
                .next_if
                .as_ref()
                .map(|next| ir::If::lower(next, context))
                .transpose()?
                .map(Box::new),
            else_ref: node.else_span.as_ref().map(|span| context.src_ref(span)),
            body_else: node
                .else_body
                .as_ref()
                .map(|body| ir::Body::lower(body, context))
                .transpose()?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower for ir::ExpressionStatement {
    type AstNode = ast::ExpressionStatement;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::ExpressionStatement {
            src_ref: context.src_ref(&node.span),
            attribute_list: ir::AttributeList::lower(&node.attributes, context)?,
            expression: ir::Expression::lower(&node.expr, context)?,
        })
    }
}

impl Lower for ir::Statement {
    type AstNode = ast::Statement;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(match node {
            ast::Statement::InlineModule(module) => ir::Statement::Module(std::rc::Rc::new(
                ir::ModuleDefinition::from_ast_inline(module, context)?,
            )),
            ast::Statement::FileModule(module) => ir::Statement::Module(std::rc::Rc::new(
                ir::ModuleDefinition::from_ast_file(module, context)?,
            )),
            ast::Statement::Use(statement) => {
                ir::Statement::Use(ir::UseStatement::lower(statement, context)?)
            }
            ast::Statement::Expression(ast::ExpressionStatement {
                expr: ast::Expression::If(if_statement),
                ..
            }) => ir::Statement::If(ir::If::lower(if_statement, context)?),
            ast::Statement::Expression(statement) => {
                ir::Statement::Expression(ir::ExpressionStatement::lower(statement, context)?)
            }
            ast::Statement::Workbench(w) => {
                ir::Statement::Workbench(<std::rc::Rc<ir::WorkbenchDefinition>>::lower(w, context)?)
            }
            ast::Statement::Function(f) => ir::Statement::Function(std::rc::Rc::new(
                ir::FunctionDefinition::lower(f, context)?,
            )),
            ast::Statement::Init(i) => {
                ir::Statement::Init(std::rc::Rc::new(ir::InitDefinition::lower(i, context)?))
            }
            ast::Statement::Return(r) => {
                ir::Statement::Return(ir::ReturnStatement::lower(r, context)?)
            }
            ast::Statement::InnerAttribute(a) => {
                ir::Statement::InnerAttribute(ir::Attribute::lower(a, context)?)
            }
            ast::Statement::InnerDocComment(i) => {
                ir::Statement::InnerDocComment(ir::InnerDocComment::lower(i, context)?)
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

impl Lower for ir::ReturnStatement {
    type AstNode = ast::Return;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::ReturnStatement {
            keyword_ref: context.src_ref(&node.keyword_span),
            result: node
                .value
                .as_ref()
                .map(|res| ir::Expression::lower(res, context))
                .transpose()?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower for ir::StatementList {
    type AstNode = ast::StatementList;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        let mut statements = Vec::new();
        node.statements.iter().try_for_each(|(statement, _)| {
            statements.push(ir::Statement::lower(statement, context)?);
            Ok(())
        })?;

        if let Some(tail) = &node.tail {
            statements.push(ir::Statement::Expression(ir::ExpressionStatement::lower(
                tail, context,
            )?));
        }

        Ok(ir::StatementList(statements))
    }
}
