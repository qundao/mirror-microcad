// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

fn _formatted_test_case(name: &str) {
    use microcad_lang_format::{FormatConfig, format};
    let source = std::fs::read_to_string(format!("tests/test_cases/formatted/{name}.µcad"))
        .expect("No errors");
    let ast = microcad_syntax::parse(&source).expect("No errors").ast;

    pretty_assertions::assert_eq!(
        source,
        format(&ast, &FormatConfig::default()),
        "Format error:\n{ast:#?}",
    );
}

fn _unformatted_test_case(name: &str) {
    use microcad_lang_format::{FormatConfig, format};
    let source = std::fs::read_to_string(format!("tests/test_cases/unformatted/{name}.µcad"))
        .expect("No errors");
    let ast = microcad_syntax::parse(&source).expect("No errors").ast;
    insta::assert_snapshot!(name, format(&ast, &FormatConfig::default()))
}

// A test case whose format output does not change.
macro_rules! formatted_test_case {
    ($name:ident) => {
        #[test]
        fn $name() {
            _formatted_test_case(stringify!($name));
        }
    };
}

// A test case whose format output does not change.
macro_rules! unformatted_test_case {
    ($name:ident) => {
        #[test]
        fn $name() {
            _unformatted_test_case(stringify!($name));
        }
    };
}

formatted_test_case!(statement_list_whitespace);
formatted_test_case!(array);
formatted_test_case!(tuple);
formatted_test_case!(body);
formatted_test_case!(expression);
formatted_test_case!(expression_if);
formatted_test_case!(statements);
formatted_test_case!(workbench);
formatted_test_case!(extras);
formatted_test_case!(extras_module);
formatted_test_case!(extras_multiline_comment);
formatted_test_case!(extras_workbench);
formatted_test_case!(definition);

unformatted_test_case!(init);
