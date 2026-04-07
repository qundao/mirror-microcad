// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

macro_rules! test_case {
    ($name:ident) => {
        #[test]
        fn $name() {
            use microcad_lang_format::{FormatConfig, format_str};
            let name = stringify!($name);
            let source = std::fs::read_to_string(format!("tests/test_cases/{name}.µcad"))
                .expect("No errors");
            insta::assert_snapshot!(
                name,
                format_str(&source, &FormatConfig::default()).expect("No errors")
            );
        }
    };
}

test_case!(workbench);
//test_case!(builder);
test_case!(extras);
