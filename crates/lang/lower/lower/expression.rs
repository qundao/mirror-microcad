// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};

use microcad_lang_base::{Identifier, Refer};
use microcad_syntax::ast;

impl FromAst for ir::RangeFirst {
    type AstNode = ast::ArrayItem;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
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
            return Err(LowerError::InvalidRangeType {
                src_ref: context.src_ref(&node.expression.span()),
            });
        }
        Ok(ir::RangeFirst(Box::new(ir::Expression::from_ast(
            &node.expression,
            context,
        )?)))
    }
}

impl FromAst for ir::RangeLast {
    type AstNode = ast::ArrayItem;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
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
            return Err(LowerError::InvalidRangeType {
                src_ref: context.src_ref(&node.expression.span()),
            });
        }
        Ok(ir::RangeLast(Box::new(ir::Expression::from_ast(
            &node.expression,
            context,
        )?)))
    }
}

impl FromAst for ir::RangeExpression {
    type AstNode = ast::ArrayRangeExpression;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::RangeExpression {
            first: ir::RangeFirst::from_ast(&node.start, context)?,
            last: ir::RangeLast::from_ast(&node.end, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for ir::ListExpression {
    type AstNode = ast::ArrayListExpression;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        node.items
            .iter()
            .map(|item| ir::Expression::from_ast(&item.expression, context))
            .collect::<Result<ir::ListExpression, _>>()
    }
}

impl FromAst for ir::Marker {
    type AstNode = ast::Identifier;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Marker {
            id: Identifier::from_ast(node, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl FromAst for ir::Expression {
    type AstNode = ast::Expression;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(match node {
            ast::Expression::Call(expr) => ir::Expression::Call(ir::Call::from_ast(expr, context)?),
            ast::Expression::Bracketed(expr, _) => ir::Expression::from_ast(expr, context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                span,
                ..
            }) => ir::Expression::FormatString(ir::FormatString(Refer::new(
                vec![ir::FormatStringInner::String(Refer::new(
                    s.content.clone(),
                    context.src_ref(&s.span),
                ))],
                context.src_ref(span),
            ))),
            ast::Expression::Literal(expr) => {
                ir::Expression::Literal(ir::Literal::from_ast(expr, context)?)
            }
            ast::Expression::String(s) => {
                ir::Expression::FormatString(ir::FormatString::from_ast(s, context)?)
            }
            ast::Expression::Tuple(t) => {
                ir::Expression::TupleExpression(ir::TupleExpression::from_ast(t, context)?)
            }
            ast::Expression::ArrayRange(a) => {
                ir::Expression::ArrayExpression(ir::ArrayExpression {
                    inner: ir::ArrayExpressionInner::Range(ir::RangeExpression::from_ast(
                        a, context,
                    )?),
                    unit: a
                        .unit
                        .as_ref()
                        .map(|unit| ir::Unit::from_ast(unit, context))
                        .transpose()?
                        .unwrap_or_default(),
                    src_ref: context.src_ref(&a.span),
                })
            }
            ast::Expression::ArrayList(a) => ir::Expression::ArrayExpression(ir::ArrayExpression {
                inner: ir::ArrayExpressionInner::List(ir::ListExpression::from_ast(a, context)?),
                unit: a
                    .unit
                    .as_ref()
                    .map(|ty| ir::Unit::from_ast(ty, context))
                    .transpose()?
                    .unwrap_or_default(),
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::QualifiedName(n) => {
                ir::Expression::QualifiedName(ir::QualifiedName::from_ast(n, context)?)
            }
            ast::Expression::Marker(m) => ir::Expression::Marker(ir::Marker::from_ast(m, context)?),
            ast::Expression::BinaryOperation(binop) => ir::Expression::BinaryOp {
                lhs: Box::new(ir::Expression::from_ast(&binop.lhs, context)?),
                rhs: Box::new(ir::Expression::from_ast(&binop.rhs, context)?),
                op: Refer::new(
                    binop.operation.operation.as_str().into(),
                    context.src_ref(&binop.operation.span),
                ),
                src_ref: context.src_ref(&binop.span),
            },
            ast::Expression::UnaryOperation(unop) => ir::Expression::UnaryOp {
                rhs: Box::new(ir::Expression::from_ast(&unop.rhs, context)?),
                op: Refer::new(
                    unop.operation.operation.as_str().into(),
                    context.src_ref(&unop.operation.span),
                ),
                src_ref: context.src_ref(&unop.span),
            },
            ast::Expression::Body(b) => ir::Expression::Body(ir::Body::from_ast(b, context)?),
            ast::Expression::ElementAccess(access) => access.element_chain.iter().try_fold(
                ir::Expression::from_ast(&access.value, context)?,
                |acc, element| {
                    use ast::ElementInner::*;

                    Ok(match &element.inner {
                        Attribute(a) => ir::Expression::AttributeAccess(
                            Box::new(acc),
                            Identifier::from_ast(a, context)?,
                            context.src_ref(&access.span),
                        ),
                        Tuple(t) => ir::Expression::PropertyAccess(
                            Box::new(acc),
                            Identifier::from_ast(t, context)?,
                            context.src_ref(&access.span),
                        ),
                        Method(m) => ir::Expression::MethodCall(
                            Box::new(acc),
                            ir::MethodCall::from_ast(m, context)?,
                            context.src_ref(&access.span),
                        ),
                        ArrayElement(e) => ir::Expression::ArrayElementAccess(
                            Box::new(acc),
                            Box::new(ir::Expression::from_ast(e, context)?),
                            context.src_ref(&access.span),
                        ),
                    })
                },
            )?,
            ast::Expression::If(i) => {
                ir::Expression::If(Box::new(ir::IfStatement::from_ast(i, context)?))
            }
            ast::Expression::Error(span) => {
                return Err(LowerError::InvalidExpression {
                    src_ref: context.src_ref(span),
                });
            }
        })
    }
}

impl FromAst for ir::TupleExpression {
    type AstNode = ast::TupleExpression;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        let mut args = ir::ArgumentList::default();
        for value in &node.values {
            args.value
                .try_push(ir::Argument {
                    id: value
                        .name
                        .as_ref()
                        .map(|name| Identifier::from_ast(name, context))
                        .transpose()?,
                    expression: ir::Expression::from_ast(&value.value, context)?,
                    src_ref: context.src_ref(&value.span),
                })
                .map_err(|(previous, id)| LowerError::DuplicateArgument { previous, id })?;
        }

        Ok(ir::TupleExpression {
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
        let statement = ast
            .statements
            .statements
            .first()
            .map(|(statement, _)| match statement {
                ast::Statement::Expression(expr) => expr,
                _ => panic!("Invalid tuple expr"),
            })
            .or(ast.statements.tail.as_deref())
            .expect("empty source");

        let tuple = match &statement.expression {
            ast::Expression::Tuple(tuple) => tuple,
            _ => panic!("No tuple"),
        };
        TupleExpression::from_ast(&tuple, &context).unwrap()
    }};
}
