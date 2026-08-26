// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating functions.

use microcad_builtin::__mu;
use microcad_lang_base::{Identifier, SrcRef, SymbolId, ToCompactString};
use microcad_lang_eval::{CallTrait, EvalContext};
use microcad_lang_types::{ArgumentValueList, Type, Value, argument_value};
use microcad_package::symbol::{
    Attributes, ConstantValue, Function, FunctionExpression, FunctionStatement, Parameter, Path,
    function::{self, Argument, ArgumentList, Call, If, Scope},
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
    microcad_package::symbol::function::Scope {
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
    let f = function::Function {
        signature: function::FunctionSignature::new(vec![Parameter::new("a", Type::Integer)])
            .with_return_type(Type::Integer),
        statements: statements(
            [function::ReturnStatement {
                expr: Some(name_expr("a")),
                keyword_src_ref: SrcRef::none(),
                src_ref: SrcRef::none(),
            }]
            .into_iter(),
        ),
        attr: Attributes::default(),
    };

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
    let f = function::Function {
        signature: function::FunctionSignature::new(vec![
            Parameter::new("a", Type::Integer),
            Parameter::new("b", Type::Integer),
        ])
        .with_return_type(Type::Integer),
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
        attr: Attributes::default(),
    };

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
    let f = Function {
        signature: function::FunctionSignature::new(vec![
            Parameter::new("a", Type::Integer),
            Parameter::new("b", Type::Integer),
        ])
        .with_return_type(Type::Integer),
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
                    [FunctionStatement::Tail(ConstantValue::from_value(2).into())].into_iter(),
                )
                .into(),
                else_ref: None,
                body_else: Some(
                    scope(
                        [FunctionStatement::Tail(ConstantValue::from_value(4).into())].into_iter(),
                    )
                    .into(),
                ),
                next_if: None,
            }]
            .into_iter(),
        ),
        attr: Attributes::default(),
    };

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
