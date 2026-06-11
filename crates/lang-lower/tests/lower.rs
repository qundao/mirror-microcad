// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base as base;
use microcad_lang_lower as lower;
use microcad_lang_parse as parse;

fn test(name: &str) {
    let path_string = format!("tests/test_cases/{name}.{}", base::MICROCAD_EXTENSION);
    let path = std::path::PathBuf::from(path_string);
    let abs_path = path.canonicalize().expect("No error");

    let source = base::Source {
        url: base::Url::from_file_path(abs_path).expect("No error"),
        line_offset: 0,
        code: base::Hashed::new(std::fs::read_to_string(path).expect("No error")),
    };

    use microcad_lang_parse::Parse;
    let ast = parse::ast::Source::parse(&parse::ParseContext::new(source.code()))
        .expect("No parse errors");

    use microcad_lang_lower::Lower;
    let mut context = lower::LowerContext::new(&ast.code);
    let ir = lower::ir::Source::lower(&ast, &mut context).expect("No lower errors");

    insta::assert_debug_snapshot!(name, ir);
}

macro_rules! test_case {
    ($name:ident) => {
        #[test]
        fn $name() {
            test(stringify!($name));
        }
    };
}

test_case!(assignments_const_const_assignment_building_code);
