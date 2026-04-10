// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

// A test case whose format output does not change.
macro_rules! formatted_test_case {
    ($name:ident) => {
        #[test]
        fn $name() {
            use microcad_lang_format::{FormatConfig, format};
            let name = stringify!($name);
            let source = std::fs::read_to_string(format!("tests/test_cases/formatted/{name}.µcad"))
                .expect("No errors");
            let source_file = microcad_syntax::parse_str(&source).expect("No errors");

            pretty_assertions::assert_eq!(
                source,
                format(&source_file, &FormatConfig::default()),
                "Format error:\n{source_file:#?}",
            );
        }
    };
}

formatted_test_case!(statement_list_whitespace);
formatted_test_case!(array);
formatted_test_case!(tuple);
formatted_test_case!(body);
formatted_test_case!(expression);
formatted_test_case!(statements);
formatted_test_case!(workbench);
formatted_test_case!(extras);
