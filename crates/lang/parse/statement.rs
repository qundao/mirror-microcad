// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};
use microcad_syntax::ast;

impl FromAst for Assignment {
    type AstNode = ast::Assignment;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Assignment {
            doc: node
                .doc
                .as_ref()
                .map(|doc| DocBlock::from_ast(doc, context))
                .transpose()?,
            visibility: node
                .visibility
                .as_ref()
                .map(|v| Visibility::from_ast(v, context))
                .transpose()?
                .unwrap_or_default(),
            id: Identifier::from_ast(&node.name, context)?,
            qualifier: node
                .qualifier
                .as_ref()
                .map(|q| Qualifier::from_ast(q, context))
                .transpose()?
                .unwrap_or_default(),
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| TypeAnnotation::from_ast(ty, context))
                .transpose()?,
            expression: Expression::from_ast(&node.value, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for AssignmentStatement {
    type AstNode = ast::Assignment;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(AssignmentStatement {
            attribute_list: AttributeList::from_ast(&node.attributes, context)?,
            assignment: Rc::new(Assignment::from_ast(node, context)?),
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for IfStatement {
    type AstNode = ast::If;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(IfStatement {
            cond: Expression::from_ast(&node.condition, context)?,
            body: Body::from_ast(&node.body, context)?,
            next_if: node
                .next_if
                .as_ref()
                .map(|next| IfStatement::from_ast(next, context))
                .transpose()?
                .map(Box::new),
            body_else: node
                .else_body
                .as_ref()
                .map(|body| Body::from_ast(body, context))
                .transpose()?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for ExpressionStatement {
    type AstNode = ast::ExpressionStatement;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(ExpressionStatement {
            src_ref: context.src_ref(&node.span),
            attribute_list: AttributeList::from_ast(&node.attributes, context)?,
            expression: Expression::from_ast(&node.expression, context)?,
        })
    }
}

impl FromAst for Statement {
    type AstNode = ast::Statement;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(match node {
            ast::Statement::Module(module) => {
                Statement::Module(Rc::new(ModuleDefinition::from_ast(module, context)?))
            }
            ast::Statement::Use(statement) => {
                Statement::Use(UseStatement::from_ast(statement, context)?)
            }
            ast::Statement::Expression(ast::ExpressionStatement { expression: ast::Expression::If(if_statement), .. }) => {
                Statement::If(IfStatement::from_ast(if_statement, context)?)
            }
            ast::Statement::Expression(statement) => {
                Statement::Expression(ExpressionStatement::from_ast(statement, context)?)
            }
            ast::Statement::Workbench(w) => {
                Statement::Workbench(<Rc<WorkbenchDefinition>>::from_ast(w, context)?)
            }
            ast::Statement::Function(f) => {
                Statement::Function(Rc::new(FunctionDefinition::from_ast(f, context)?))
            }
            ast::Statement::Init(i) => {
                Statement::Init(Rc::new(InitDefinition::from_ast(i, context)?))
            }
            ast::Statement::Return(r) => Statement::Return(ReturnStatement::from_ast(r, context)?),
            ast::Statement::InnerAttribute(a) => {
                Statement::InnerAttribute(Attribute::from_ast(a, context)?)
            }
            ast::Statement::Assignment(a) => {
                Statement::Assignment(AssignmentStatement::from_ast(a, context)?)
            }
            ast::Statement::Comment(_) => unreachable!("comments are filtered out"),
            ast::Statement::Error(span) => {
                return Err(ParseError::InvalidStatement {
                    src_ref: context.src_ref(span),
                });
            }
        })
    }
}

impl FromAst for ReturnStatement {
    type AstNode = ast::Return;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(ReturnStatement {
            result: node
                .value
                .as_ref()
                .map(|res| Expression::from_ast(res, context))
                .transpose()?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for StatementList {
    type AstNode = ast::StatementList;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(StatementList(
            node.statements
                .iter()
                .chain(node.tail.iter().map(|tail| tail.as_ref()))
                .filter(|statement| !matches!(statement, ast::Statement::Comment(_)))
                .map(|statement| Statement::from_ast(statement, context))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl FromAst for Qualifier {
    type AstNode = ast::AssignmentQualifier;

    fn from_ast(node: &Self::AstNode, _context: &ParseContext) -> Result<Self, ParseError> {
        Ok(match node {
            ast::AssignmentQualifier::Const => Qualifier::Const,
            ast::AssignmentQualifier::Prop => Qualifier::Prop,
        })
    }
}
