// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;
use microcad_syntax::ast::{Element, LiteralKind};

impl FromAst for RangeFirst {
    type AstNode = ast::ArrayItem;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        if matches!(
            node.expression,
            ast::Expression::Literal(
                ast::Literal {
                    literal: ast::LiteralKind::Float(_)
                        | ast::LiteralKind::String(_)
                        | ast::LiteralKind::Quantity(_)
                        | ast::LiteralKind::Bool(_),
                    ..
                },
                ..
            )
        ) {
            return Err(ParseError::InvalidRangeType {
                src_ref: context.src_ref(&node.expression.span()),
            });
        }
        Ok(RangeFirst(Box::new(Expression::from_ast(
            &node.expression,
            context,
        )?)))
    }
}

impl FromAst for RangeLast {
    type AstNode = ast::ArrayItem;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        if matches!(
            node.expression,
            ast::Expression::Literal(
                ast::Literal {
                    literal: ast::LiteralKind::Float(_)
                        | ast::LiteralKind::String(_)
                        | ast::LiteralKind::Quantity(_)
                        | ast::LiteralKind::Bool(_),
                    ..
                },
                ..
            )
        ) {
            return Err(ParseError::InvalidRangeType {
                src_ref: context.src_ref(&node.expression.span()),
            });
        }
        Ok(RangeLast(Box::new(Expression::from_ast(
            &node.expression,
            context,
        )?)))
    }
}

impl FromAst for RangeExpression {
    type AstNode = ast::ArrayRangeExpression;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(RangeExpression {
            first: RangeFirst::from_ast(&node.start, context)?,
            last: RangeLast::from_ast(&node.end, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for ListExpression {
    type AstNode = ast::ArrayListExpression;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        node.items
            .iter()
            .map(|item| Expression::from_ast(&item.expression, context))
            .collect::<Result<ListExpression, _>>()
    }
}

impl FromAst for Marker {
    type AstNode = ast::Identifier;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Marker {
            id: Identifier::from_ast(node, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for Expression {
    type AstNode = ast::Expression;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(match node {
            ast::Expression::Call(expr) => Expression::Call(Call::from_ast(expr, context)?),
            ast::Expression::Literal(ast::Literal {
                literal: LiteralKind::String(s),
                span,
                ..
            }) => Expression::FormatString(FormatString(Refer::new(
                vec![FormatStringInner::String(Refer::new(
                    s.content.clone(),
                    context.src_ref(&s.span),
                ))],
                context.src_ref(span),
            ))),
            ast::Expression::Literal(expr) => {
                Expression::Literal(Literal::from_ast(expr, context)?)
            }
            ast::Expression::String(s) => {
                Expression::FormatString(FormatString::from_ast(s, context)?)
            }
            ast::Expression::Tuple(t) => {
                Expression::TupleExpression(TupleExpression::from_ast(t, context)?)
            }
            ast::Expression::ArrayRange(a) => Expression::ArrayExpression(ArrayExpression {
                inner: ArrayExpressionInner::Range(RangeExpression::from_ast(a, context)?),
                unit: a
                    .ty
                    .as_ref()
                    .map(|ty| Unit::from_ast(ty, context))
                    .transpose()?
                    .unwrap_or_default(),
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::ArrayList(a) => Expression::ArrayExpression(ArrayExpression {
                inner: ArrayExpressionInner::List(ListExpression::from_ast(a, context)?),
                unit: a
                    .ty
                    .as_ref()
                    .map(|ty| Unit::from_ast(ty, context))
                    .transpose()?
                    .unwrap_or_default(),
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::QualifiedName(n) => {
                Expression::QualifiedName(QualifiedName::from_ast(n, context)?)
            }
            ast::Expression::Marker(m) => Expression::Marker(Marker::from_ast(m, context)?),
            ast::Expression::BinaryOperation(binop) => Expression::BinaryOp {
                lhs: Box::new(Expression::from_ast(&binop.lhs, context)?),
                rhs: Box::new(Expression::from_ast(&binop.rhs, context)?),
                op: binop.operation.as_str().into(),
                src_ref: context.src_ref(&binop.span),
            },
            ast::Expression::UnaryOperation(unop) => Expression::UnaryOp {
                rhs: Box::new(Expression::from_ast(&unop.rhs, context)?),
                op: unop.operation.as_str().into(),
                src_ref: context.src_ref(&unop.span),
            },
            ast::Expression::Block(b) => Expression::Body(Body::from_ast(b, context)?),
            ast::Expression::ElementAccess(access) => match &access.element {
                Element::Attribute(a) => Expression::AttributeAccess(
                    Box::new(Expression::from_ast(&access.value, context)?),
                    Identifier::from_ast(a, context)?,
                    context.src_ref(&access.span),
                ),
                Element::Tuple(t) => Expression::PropertyAccess(
                    Box::new(Expression::from_ast(&access.value, context)?),
                    Identifier::from_ast(t, context)?,
                    context.src_ref(&access.span),
                ),
                Element::Method(m) => Expression::MethodCall(
                    Box::new(Expression::from_ast(&access.value, context)?),
                    MethodCall::from_ast(m, context)?,
                    context.src_ref(&access.span),
                ),
                Element::ArrayElement(e) => Expression::ArrayElementAccess(
                    Box::new(Expression::from_ast(&access.value, context)?),
                    Box::new(Expression::from_ast(e, context)?),
                    context.src_ref(&access.span),
                ),
            },
            ast::Expression::If(i) => Expression::If(Box::new(IfStatement::from_ast(i, context)?)),
            ast::Expression::Error(span) => {
                return Err(ParseError::InvalidExpression {
                    src_ref: context.src_ref(span),
                });
            }
        })
    }
}

impl FromAst for TupleExpression {
    type AstNode = ast::TupleExpression;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let mut args = ArgumentList::default();
        for value in &node.values {
            args.value
                .try_push(Argument {
                    id: value
                        .name
                        .as_ref()
                        .map(|name| Identifier::from_ast(name, context))
                        .transpose()?,
                    expression: Expression::from_ast(&value.value, context)?,
                    src_ref: context.src_ref(&value.span),
                })
                .map_err(|(previous, id)| ParseError::DuplicateArgument {
                    previous,
                    id,
                })?;
        }

        Ok(TupleExpression {
            args,
            src_ref: context.src_ref(&node.span),
        })
    }
}

/// Create TupleExpression from µcad code
#[cfg(test)]
#[macro_export]
macro_rules! tuple_expression {
    ($code:literal) => {{
        use microcad_syntax::ast;
        use $crate::parser::FromAst;
        let context = $crate::parser::ParseContext::new($code);
        let ast = $crate::parse::build_ast($code, &context).unwrap();
        let statement = ast.statements.statements.first().or(ast.statements.tail.as_deref()).expect("empty source");
        let tuple_expression = match statement {
            ast::Statement::Expression(ast::ExpressionStatement {
                expression: ast::Expression::Tuple(tuple),
                ..
            }) => tuple,
            _ => panic!("non tuple source"),
        };
        TupleExpression::from_ast(&tuple_expression, &context).unwrap()
    }};
}
