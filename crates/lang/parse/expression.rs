// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};
use microcad_syntax::ast;
use microcad_syntax::ast::{Element, LiteralKind};

impl Parse for RangeFirst {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self(Box::new(
            pair.find(Rule::expression)
                .or(pair
                    .find(Rule::integer_literal)
                    .map(|i| Expression::Literal(Literal::Integer(i))))
                .expect("Expression"),
        )))
    }
}

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

impl Parse for RangeLast {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self(Box::new(
            pair.find(Rule::expression)
                .or(pair
                    .find(Rule::integer_literal)
                    .map(|i| Expression::Literal(Literal::Integer(i))))
                .expect("Expression"),
        )))
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

impl Parse for RangeExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self {
            first: pair.find(Rule::range_start).expect("Range start"),
            last: pair.find(Rule::range_end).expect("Range end"),
            src_ref: pair.src_ref(),
        })
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

impl Parse for ListExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        pair.inner()
            .filter_map(|pair| match pair.as_rule() {
                Rule::expression => Some(Expression::parse(pair)),
                _ => None,
            })
            .collect::<Result<Vec<_>, _>>()
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

impl Parse for ArrayExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self {
            inner: pair
                .find(Rule::range_expression)
                .map(ArrayExpressionInner::Range)
                .or(pair
                    .find(Rule::list_expression)
                    .map(ArrayExpressionInner::List))
                .unwrap_or_default(),
            unit: pair.find(Rule::unit).unwrap_or_default(),
            src_ref: pair.clone().into(),
        })
    }
}

impl Parse for Marker {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::marker);
        Ok(Self {
            id: Identifier::parse(pair.inner().next().expect(INTERNAL_PARSE_ERROR))?,
            src_ref: pair.src_ref(),
        })
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

lazy_static::lazy_static! {
    /// Expression parser
    static ref PRATT_PARSER: pest::pratt_parser::PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc, Op,PrattParser};
        use Assoc::*;
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(or, Left) | Op::infix(and, Left))
            .op(Op::infix(equal, Left) | Op::infix(not_equal, Left))
            .op(Op::infix(greater_than, Left) | Op::infix(less_than, Left))
            .op(Op::infix(less_equal, Left) | Op::infix(greater_equal, Left))
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(r#union, Left) | Op::infix(intersect, Left))
            .op(Op::infix(power_xor, Left))
            .op(Op::infix(near, Left))
            .op(Op::prefix(unary_minus))
            .op(Op::prefix(unary_plus))
            .op(Op::prefix(unary_not))
            .op(Op::postfix(method_call))
            .op(Op::postfix(array_element_access))
            .op(Op::postfix(tuple_element_access))
            .op(Op::postfix(attribute_access))
    };
}

impl Expression {
    /// Generate literal from string
    pub fn literal_from_str(s: &str) -> ParseResult<Self> {
        use std::str::FromStr;
        if s.len() > 1 && s.starts_with('"') && s.ends_with('"') {
            Ok(Self::FormatString(FormatString::from_str(s)?))
        } else {
            Ok(Self::Literal(Literal::from_str(s)?))
        }
    }
}

impl Parse for Expression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::expression);

        PRATT_PARSER
            .map_primary(|primary| {
                match (
                    Pair::new(primary.clone(), pair.source_hash()),
                    primary.as_rule(),
                ) {
                    (primary, Rule::literal) => Ok(Self::Literal(Literal::parse(primary)?)),
                    (primary, Rule::expression) => Ok(Self::parse(primary)?),
                    (primary, Rule::array_expression) => {
                        Ok(Self::ArrayExpression(ArrayExpression::parse(primary)?))
                    }
                    (primary, Rule::tuple_expression) => {
                        Ok(Self::TupleExpression(TupleExpression::parse(primary)?))
                    }
                    (primary, Rule::format_string) => {
                        Ok(Self::FormatString(FormatString::parse(primary)?))
                    }
                    (primary, Rule::body) => Ok(Self::Body(Body::parse(primary)?)),
                    (primary, Rule::if_statement) => {
                        let statement = IfStatement::parse(primary)?;
                        if !statement.is_complete() {
                            Err(ParseError::IncompleteIfExpression(statement.src_ref()))
                        } else {
                            Ok(Self::If(Box::new(statement)))
                        }
                    }
                    (primary, Rule::call) => Ok(Self::Call(Call::parse(primary)?)),
                    (primary, Rule::qualified_name) => {
                        Ok(Self::QualifiedName(QualifiedName::parse(primary)?))
                    }
                    (primary, Rule::marker) => Ok(Self::Marker(Marker::parse(primary)?)),
                    rule => unreachable!(
                        "Expression::parse expected atom, found {:?} {:?}",
                        rule,
                        pair.as_span().as_str()
                    ),
                }
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => "+",
                    Rule::subtract => "-",
                    Rule::multiply => "*",
                    Rule::divide => "/",
                    Rule::r#union => "|",
                    Rule::intersect => "&",
                    Rule::power_xor => "^",
                    Rule::greater_than => ">",
                    Rule::less_than => "<",
                    Rule::less_equal => "≤",
                    Rule::greater_equal => "≥",
                    Rule::equal => "==",
                    Rule::near => "~",
                    Rule::not_equal => "!=",
                    Rule::and => "&",
                    Rule::or => "|",

                    rule => unreachable!(
                        "Expression::parse expected infix operation, found {:?}",
                        rule
                    ),
                };
                Ok(Self::BinaryOp {
                    lhs: Box::new(lhs?),
                    op: op.into(),
                    rhs: Box::new(rhs?),
                    src_ref: pair.clone().into(),
                })
            })
            .map_prefix(|op, rhs| {
                let op = match op.as_rule() {
                    Rule::unary_minus => '-',
                    Rule::unary_plus => '+',
                    Rule::unary_not => '!',
                    _ => unreachable!(),
                };

                Ok(Self::UnaryOp {
                    op: op.into(),
                    rhs: Box::new(rhs?),
                    src_ref: pair.clone().into(),
                })
            })
            .map_postfix(|lhs, op| {
                match (Pair::new(op.clone(), pair.source_hash()), op.as_rule()) {
                    (op, Rule::array_element_access) => Ok(Self::ArrayElementAccess(
                        Box::new(lhs?),
                        Box::new(Self::parse(op)?),
                        pair.clone().into(),
                    )),
                    (op, Rule::attribute_access) => {
                        let op = op.inner().next().expect(INTERNAL_PARSE_ERROR);
                        Ok(Self::AttributeAccess(
                            Box::new(lhs?),
                            Identifier::parse(op)?,
                            pair.clone().into(),
                        ))
                    }
                    (op, Rule::tuple_element_access) => {
                        let op = op.inner().next().expect(INTERNAL_PARSE_ERROR);
                        match op.as_rule() {
                            Rule::identifier => Ok(Self::PropertyAccess(
                                Box::new(lhs?),
                                Identifier::parse(op)?,
                                pair.clone().into(),
                            )),
                            rule => unreachable!("Expected identifier or int, found {:?}", rule),
                        }
                    }
                    (op, Rule::method_call) => Ok(Self::MethodCall(
                        Box::new(lhs?),
                        MethodCall::parse(op)?,
                        pair.clone().into(),
                    )),
                    rule => {
                        unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                    }
                }
            })
            .parse(
                pair.pest_pair()
                    .clone()
                    .into_inner()
                    .filter(|pair| pair.as_rule() != Rule::COMMENT), // Filter comments
            )
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

impl Parse for Rc<Expression> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Rc::new(Expression::parse(pair)?))
    }
}

impl Parse for TupleExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(TupleExpression {
            args: crate::find_rule!(pair, argument_list)?,
            src_ref: pair.clone().into(),
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
                .map_err(ParseError::DuplicateArgument)?;
        }

        Ok(TupleExpression {
            args,
            src_ref: context.src_ref(&node.span),
        })
    }
}

/// Create TupleExpression from µcad code
#[macro_export]
macro_rules! tuple_expression {
    ($code:literal) => {{
        $crate::parse!(
            TupleExpression,
            $crate::parser::Rule::tuple_expression,
            $code
        )
    }};
}
