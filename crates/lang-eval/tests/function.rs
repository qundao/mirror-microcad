// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating functions.

use microcad_builtin::__mu;
use microcad_lang_base::{SrcRef, SymbolId, ToCompactString, boxed};
use microcad_lang_eval::{CallTrait, EvalContext};
use microcad_lang_resolve::{
    call_builtin,
    symbol::{
        ConstantValue, Function, FunctionExpression, FunctionStatement, Parameter, Path, function,
    },
};
use microcad_lang_types::{ArgumentValueList, Type, Value, argument_value};

fn name_expr(name: &str) -> FunctionExpression {
    FunctionExpression::Path(Path::Resolved(SymbolId::Local(name.to_compact_string())))
}

fn scope(a: impl Iterator<Item = FunctionStatement>) -> function::Scope {
    function::Scope {
        statements: boxed(a),
        src_ref: SrcRef::none(),
    }
}

#[test]
fn return_a() {
    let f = function::Function::new(vec![Parameter::new("a", Type::Integer)])
        .with_return_type(Type::Integer)
        .with_statements([function::ReturnStatement {
            expr: Some(name_expr("a")),
            keyword_src_ref: SrcRef::none(),
            src_ref: SrcRef::none(),
        }
        .into()]);

    let mut context = EvalContext::new();
    let result = f
        .call(
            &ArgumentValueList::from_iter([argument_value!(a = 2)]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(2))
}

#[test]
fn add() {
    let f = function::Function::new(vec![
        Parameter::new("a", Type::Integer),
        Parameter::new("b", Type::Integer),
    ])
    .with_return_type(Type::Integer)
    .with_statements([FunctionStatement::Tail(
        call_builtin!(core::add(lhs = name_expr("a"), rhs = name_expr("b"))).into(),
    )]);

    let mut context = EvalContext::new();
    let result = f
        .call(
            &ArgumentValueList::from_iter([argument_value!(a = 1), argument_value!(b = 3)]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(4));
}

#[test]
fn if_a_greater_than() {
    let f = Function::new(vec![
        Parameter::new("a", Type::Integer),
        Parameter::new("b", Type::Integer),
    ])
    .with_return_type(Type::Integer)
    .with_statements([function::FunctionIf {
        src_ref: SrcRef::none(),
        if_ref: SrcRef::none(),
        cond: FunctionExpression::Call(call_builtin!(core::gt(
            lhs = name_expr("a"),
            rhs = name_expr("b"),
        )))
        .into(),
        body: scope([FunctionStatement::Tail(ConstantValue::new(2).into())].into_iter()).into(),
        else_ref: None,
        body_else: Some(
            scope([FunctionStatement::Tail(ConstantValue::new(4).into())].into_iter()).into(),
        ),
        next_if: None,
    }
    .into()]);

    let mut context = EvalContext::new();
    let result = f
        .call(
            &ArgumentValueList::from_iter([argument_value!(a = 1), argument_value!(b = 3)]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(4));

    let mut context = EvalContext::new();
    let result = f
        .call(
            &ArgumentValueList::from_iter([argument_value!(a = 3), argument_value!(b = 1)]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(2))
}
