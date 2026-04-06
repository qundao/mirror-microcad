// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_markdown::Markdown;

macro_rules! test_case {
    ($name:ident) => {
        #[test]
        fn $name() {
            let name = stringify!($name);
            let path = format!("tests/test_cases/{name}.md");
            insta::assert_snapshot!(name, Markdown::load(path).expect("No errors"));
        }
    };
}

test_case!(basic);
test_case!(paragraphs);
test_case!(codeblock);
