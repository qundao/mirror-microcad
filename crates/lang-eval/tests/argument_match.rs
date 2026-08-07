// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_eval::{argument_value, arguments};
use microcad_lang_types::{ArgumentValueList, Length, Scalar, Type, function_type, tuple};

#[test]
fn argument_matching() {
    let ty =
        function_type!(a: Type::scalar(), b: Type::length(), c: Type::scalar(), d: Type::length());
    let defaults = tuple!(d = Length::mm(4.0));

    let arguments: ArgumentValueList = [
        argument_value!(a: Scalar = Scalar::from_num(1.0)),
        argument_value!(b: Length = Length::mm(2.0)),
        argument_value!(Scalar = Scalar::from_num(3.0)),
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
    let ty = function_type!(x: Type::scalar(), y: Type::scalar(), z: Type::scalar());
    let arguments: ArgumentValueList = [
        argument_value!(x: Scalar = Scalar::from_num(1.0)),
        argument_value!(Length = Length::mm(1.0)),
    ]
    .into_iter()
    .collect();
    assert!(microcad_lang_eval::find_match(&arguments, &ty, &Default::default()).is_err());
}
