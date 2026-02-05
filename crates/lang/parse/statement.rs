// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};
use microcad_syntax::ast;

impl Parse for Assignment {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self::new(
            crate::find_rule_opt!(pair, doc_block)?,
            crate::find_rule!(pair, visibility)?,
            crate::find_rule!(pair, qualifier)?,
            crate::find_rule!(pair, identifier)?,
            crate::find_rule_opt!(pair, r#type)?,
            crate::find_rule_exact!(pair, expression)?,
            pair.into(),
        ))
    }
}

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

impl Parse for Rc<Assignment> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Rc::new(Assignment::parse(pair)?))
    }
}

impl Parse for AssignmentStatement {
    fn parse(pair: Pair) -> crate::parse::ParseResult<Self> {
        Ok(Self {
            attribute_list: crate::find_rule!(pair, attribute_list)?,
            assignment: crate::find_rule_opt!(pair, assignment)?.expect("Assignment"),
            src_ref: pair.into(),
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

impl Parse for IfStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut cond = Default::default();
        let mut body = None;
        let mut body_else = None;
        let mut next_if = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => cond = Expression::parse(pair)?,
                Rule::body => {
                    if body.is_none() {
                        body = Some(Body::parse(pair)?)
                    } else {
                        body_else = Some(Body::parse(pair)?)
                    }
                }
                Rule::if_statement => {
                    if next_if.is_none() {
                        next_if = Some(Box::new(IfStatement::parse(pair)?));
                    }
                }
                rule => unreachable!("Unexpected rule in if, got {:?}", rule),
            }
        }

        let body = body.expect("Body");

        Ok(IfStatement {
            cond,
            body,
            body_else,
            next_if,
            src_ref: pair.into(),
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

impl Parse for ExpressionStatement {
    fn parse(pair: Pair) -> crate::parse::ParseResult<Self> {
        Parser::ensure_rules(
            &pair,
            &[Rule::expression_statement, Rule::final_expression_statement],
        );

        Ok(Self {
            attribute_list: crate::find_rule!(pair, attribute_list)?,
            expression: pair.find(Rule::expression).expect("Expression"),
            src_ref: pair.into(),
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

impl Parse for Statement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::statement);
        let first = pair.inner().next().expect(INTERNAL_PARSE_ERROR);
        Ok(match first.as_rule() {
            Rule::workbench_definition => Self::Workbench(Rc::<WorkbenchDefinition>::parse(first)?),
            Rule::module_definition => Self::Module(Rc::<ModuleDefinition>::parse(first)?),
            Rule::function_definition => Self::Function(Rc::<FunctionDefinition>::parse(first)?),
            Rule::init_definition => Self::Init(Rc::new(InitDefinition::parse(first)?)),

            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::return_statement => Self::Return(ReturnStatement::parse(first)?),
            Rule::if_statement => Self::If(IfStatement::parse(first)?),
            Rule::inner_attribute => Self::InnerAttribute(Attribute::parse(first)?),

            Rule::assignment_statement => Self::Assignment(AssignmentStatement::parse(first)?),
            Rule::expression_statement | Rule::final_expression_statement => {
                Self::Expression(ExpressionStatement::parse(first)?)
            }
            rule => unreachable!("Unexpected statement, got {:?} {:?}", rule, first.clone()),
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

impl Parse for ReturnStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut result = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => result = Some(Expression::parse(pair)?),
                rule => unreachable!("Unexpected rule in return, got {:?}", rule),
            }
        }

        Ok(ReturnStatement {
            result,
            src_ref: pair.into(),
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

impl Parse for StatementList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut statements = Vec::new();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::final_expression_statement => {
                    statements.push(Statement::Expression(ExpressionStatement::parse(pair)?));
                }
                Rule::statement => {
                    statements.push(Statement::parse(pair)?);
                }
                _ => {}
            }
        }

        Ok(Self(statements))
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

impl Parse for Qualifier {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::qualifier);
        match pair.as_str() {
            "prop" => Ok(Self::Prop),
            "const" => Ok(Self::Const),
            _ => Ok(Self::Value),
        }
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
