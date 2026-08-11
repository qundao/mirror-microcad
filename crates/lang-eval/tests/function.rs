// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating functions.

use microcad_builtin::__mu;
use microcad_lang_base::{Identifier, SrcRef, SymbolId, ToCompactString};
use microcad_lang_eval::{CallTrait, EvalContext, argument_value};
use microcad_lang_types::{ArgumentValueList, Integer, Type, Value, function_type, tuple};
use microcad_package::rst::{
    Function, FunctionExpression, FunctionStatement, Path,
    function::{Argument, ArgumentList, Call, If, Literal, ReturnStatement, Scope},
};

fn statements<T>(a: impl Iterator<Item = T>) -> Box<[FunctionStatement]>
where
    T: Into<FunctionStatement>,
{
    a.map(|stmt| stmt.into())
        .collect::<Vec<FunctionStatement>>()
        .into_boxed_slice()
}

fn name_expr(name: &str) -> FunctionExpression {
    FunctionExpression::Path(Path::Resolved(SymbolId::Local(name.to_compact_string())))
}

fn scope<T>(a: impl Iterator<Item = T>) -> Scope
where
    T: Into<FunctionStatement>,
{
    microcad_package::rst::function::Scope {
        statements: statements(a),
        src_ref: SrcRef::none(),
    }
}

fn arg(name: &str, expr: FunctionExpression) -> Argument {
    Argument::Named {
        name: Identifier::no_ref(name),
        expr,
        src_ref: SrcRef::none(),
    }
}

#[test]
fn return_a() {
    let f = Function {
        ty: function_type!((a: Type::Integer) -> Type::Integer),
        default_parameters: tuple!(),
        statements: statements(
            [ReturnStatement {
                expr: Some(name_expr("a")),
                keyword_src_ref: SrcRef::none(),
                src_ref: SrcRef::none(),
            }]
            .into_iter(),
        ),
    };

    let mut context = EvalContext::new();
    let result = f
        .call(
            &ArgumentValueList::from_iter([argument_value!(a: Integer = Integer::from_num(2.0))]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(2_i64))
}

#[test]
fn add() {
    let f = Function {
        ty: function_type!((a: Type::Integer, b: Type::Integer) -> Type::Integer),
        default_parameters: tuple!(),
        statements: statements(
            [FunctionStatement::Tail(
                Call {
                    path: __mu!(core::add),
                    args: ArgumentList::from_iter(
                        [arg("lhs", name_expr("a")), arg("rhs", name_expr("b"))].into_iter(),
                    ),
                    src_ref: SrcRef::none(),
                }
                .into(),
            )]
            .into_iter(),
        ),
    };

    let mut context = EvalContext::new();
    let result = f
        .call(
            &ArgumentValueList::from_iter([
                argument_value!(a: Integer = Integer::from_num(1)),
                argument_value!(b: Integer = Integer::from_num(3)),
            ]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(4_i64));
}

#[test]
fn if_a_greater_than() {
    let f = Function {
        ty: function_type!((a: Type::Integer, b: Type::Integer) -> Type::Integer),
        default_parameters: tuple!(),
        statements: statements(
            [If {
                src_ref: SrcRef::none(),
                if_ref: SrcRef::none(),
                cond: FunctionExpression::Call(Call {
                    path: __mu!(core::gt),
                    args: ArgumentList::from_iter(
                        [arg("lhs", name_expr("a")), arg("rhs", name_expr("b"))].into_iter(),
                    ),
                    src_ref: SrcRef::none(),
                })
                .into(),
                body: scope(
                    [FunctionStatement::Tail(Literal::from_value(2_i64).into())].into_iter(),
                )
                .into(),
                else_ref: None,
                body_else: Some(
                    scope([FunctionStatement::Tail(Literal::from_value(4_i64).into())].into_iter())
                        .into(),
                ),
                next_if_ref: None,
                next_if: None,
            }]
            .into_iter(),
        ),
    };

    let mut context = EvalContext::new();
    let result = f
        .call(
            &ArgumentValueList::from_iter([
                argument_value!(a: Integer = Integer::from_num(1)),
                argument_value!(b: Integer = Integer::from_num(3)),
            ]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(4_i64));

    let mut context = EvalContext::new();
    let result = f
        .call(
            &ArgumentValueList::from_iter([
                argument_value!(a: Integer = Integer::from_num(3)),
                argument_value!(b: Integer = Integer::from_num(1)),
            ]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(2_i64))
}
