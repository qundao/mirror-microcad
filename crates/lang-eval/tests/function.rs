// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating functions.

use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_eval::{ArgumentValueList, CallTrait, EvalContext, argument};
use microcad_lang_types::{Integer, Type, Value};
use microcad_package::{
    parameter,
    rst::{
        Function, FunctionExpression, FunctionStatement, ResolvedName, function::ReturnStatement,
    },
};

fn statements<T>(a: impl Iterator<Item = T>) -> Box<[FunctionStatement]>
where
    T: Into<FunctionStatement>,
{
    a.map(|stmt| stmt.into())
        .collect::<Vec<FunctionStatement>>()
        .into_boxed_slice()
}

#[test]
fn return_a() {
    let f = Function {
        parameters: [parameter!(a: Integer)].into_iter().collect(),
        return_ty: Some(Type::Integer),
        statements: statements(
            [ReturnStatement {
                expr: Some(FunctionExpression::Name(ResolvedName::Local(
                    Identifier::no_ref("a"),
                ))),
                keyword_src_ref: SrcRef::none(),
                src_ref: SrcRef::none(),
            }]
            .into_iter(),
        ),
    };

    let mut context = EvalContext::default();
    let result = f
        .call(
            &ArgumentValueList::from_iter([argument!(a: Integer = Integer::from_num(2.0))]),
            &mut context,
        )
        .expect("No eval error");

    assert_eq!(result, Value::from(2_i64))
}
