// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::Identifier;
use microcad_lang_eval::find_multi_match;
use microcad_lang_types::{
    ArgumentValue, ArgumentValueList, Length, Tuple, Type, argument_value, arguments,
    function_type, list, tuple,
};

#[test]
fn argument_matching() {
    let ty = function_type!((a: Type::scalar(), b: Type::length(), c: Type::scalar(), d: Type::length()));
    let defaults = tuple!(d = Length::mm(4.0));
    let arguments: ArgumentValueList = [
        argument_value!(a = 1.0),
        argument_value!(b = Length::mm(2.0)),
        argument_value!(3.0),
    ]
    .into_iter()
    .collect();

    let result =
        microcad_lang_eval::find_match(&arguments, &ty, &defaults).expect("expect valid arguments");

    assert_eq!(
        result,
        arguments!(a = 1.0, b = Length::mm(2.0), c = 3.0, d = Length::mm(4.0))
    );
}

#[test]
fn argument_match_fail() {
    let ty = function_type!((x: Type::scalar(), y: Type::scalar(), z: Type::scalar()));
    let arguments: ArgumentValueList = [argument_value!(x = 1.0), argument_value!(Length::mm(1.0))]
        .into_iter()
        .collect();
    assert!(microcad_lang_eval::find_match(&arguments, &ty, &Default::default()).is_err());
}

#[test]
fn multi_match_cartesian_product() {
    // Function signature: f(a: Length, b: Length) -> Length
    let fn_ty = function_type!((a: Type::length(), b: Type::length()) -> Type::length());
    let default_parameters = Tuple::default();

    // f(a = [1.0, 2.0], b = [10.0, 20.0, 30.0])
    let args: ArgumentValueList = [
        ArgumentValue::new(list![1.0, 2.0], Some(Identifier::no_ref("a"))),
        ArgumentValue::new(list![10.0, 20.0, 30.0], Some(Identifier::no_ref("b"))),
    ]
    .into_iter()
    .collect();

    let result = find_multi_match(&args, &fn_ty, &default_parameters).unwrap();

    // 2 x 3 = 6 combinations expected
    assert_eq!(result.len(), 6);

    assert_eq!(result[0], arguments!(a = 1.0, b = 10.0));
    assert_eq!(result[1], arguments!(a = 1.0, b = 20.0));
    assert_eq!(result[2], arguments!(a = 1.0, b = 30.0));
    assert_eq!(result[3], arguments!(a = 2.0, b = 10.0));
    assert_eq!(result[4], arguments!(a = 2.0, b = 20.0));
    assert_eq!(result[5], arguments!(a = 2.0, b = 30.0));
}
