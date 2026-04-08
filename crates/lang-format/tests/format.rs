// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

// A test case whose format output does not change.
macro_rules! formatted_test_case {
    ($name:ident) => {
        #[test]
        fn $name() {
            use microcad_lang_format::{FormatConfig, format_str};
            let name = stringify!($name);
            let source = std::fs::read_to_string(format!("tests/test_cases/formatted/{name}.µcad"))
                .expect("No errors");
            pretty_assertions::assert_eq!(
                source,
                format_str(&source, &FormatConfig::default()).expect("No errors")
            );
        }
    };
}

// A test case whose format output does change.
macro_rules! unformatted_test_case {
    ($name:ident) => {
        #[test]
        fn $name() {
            use microcad_lang_format::{FormatConfig, format_str};
            let name = stringify!($name);
            let source =
                std::fs::read_to_string(format!("tests/test_cases/unformatted/{name}.µcad"))
                    .expect("No errors");
            insta::assert_snapshot!(
                name,
                format_str(&source, &FormatConfig::default()).expect("No errors")
            );
        }
    };
}

formatted_test_case!(statement_list_whitespace);
