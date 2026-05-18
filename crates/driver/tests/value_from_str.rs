// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::Integer;
use microcad_lang::{
    ty::QuantityType,
    value::{Quantity, Value},
};

#[test]
fn test_value_from_str_literals() {
    let cases: Vec<(&str, Value)> = vec![
        ("0.1mm", Quantity::new(0.1, QuantityType::Length).into()), // Expecting a Quantity/Measurement
        (
            "42°",
            Quantity::new(0.7330382858376184, QuantityType::Angle).into(),
        ), // Expecting a Quantity/Angle
        ("\"Foo\"", "Foo".to_string().into()),                      // Expecting a String
        ("1", (1 as Integer).into()),                               // Expecting an Integer
        ("23.0", Quantity::new(23.0, QuantityType::Scalar).into()), // Expecting a Scalar
    ];

    for (input, output) in cases {
        let result = microcad_driver::value_from_str(input).unwrap();
        assert_eq!(result, output);
    }
}

#[test]
fn test_parse_errors() {
    // Test an invalid literal to ensure your ParseErrors/miette setup works
    let input = "[foo]";
    let result = microcad_driver::value_from_str(input);
    assert!(result.is_err());
}
