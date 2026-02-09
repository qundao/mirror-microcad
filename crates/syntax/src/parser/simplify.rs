// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ast::{Expression, Literal, LiteralKind, UnaryOperation, UnaryOperator};

/// Merge negated numeric literals into negative numeric literals
pub fn simplify_unary_op(op: UnaryOperation) -> Expression {
    match op {
        UnaryOperation {
            operation: UnaryOperator::Minus,
            rhs,
            span,
            extras,
        } => match *rhs {
            Expression::Literal(Literal {
                literal: LiteralKind::Integer(int),
                ..
            }) => Expression::Literal(Literal {
                span,
                extras,
                literal: LiteralKind::Integer(-int),
            }),
            Expression::Literal(Literal {
                literal: LiteralKind::Float(int),
                ..
            }) => Expression::Literal(Literal {
                span,
                extras,
                literal: LiteralKind::Float(-int),
            }),
            Expression::Literal(Literal {
                literal: LiteralKind::Quantity(int),
                ..
            }) => Expression::Literal(Literal {
                span,
                extras,
                literal: LiteralKind::Quantity(-int),
            }),
            rhs => Expression::UnaryOperation(UnaryOperation {
                operation: UnaryOperator::Minus,
                rhs: Box::new(rhs),
                span,
                extras,
            }),
        },
        op => Expression::UnaryOperation(op),
    }
}
