// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{
    CompilationResult, DiagRenderOptions, Identifier, MICROCAD_EXTENSION, Source,
};
use microcad_lang_lower::{self as lower, Ir, ir};
use microcad_lang_parse::{
    self as parse, Ast, Parse,
    ast::visitor::{ExpectedDiagnostic, ExpectedDiagnostics},
};

use parse::ast;

use test_that::prelude::*;

fn source_from_test_file(name: &str) -> Source {
    Source::load(format!("tests/test_cases/{name}.{}", MICROCAD_EXTENSION)).expect("No error")
}

/// Get intermediate representation and diagnostics.
fn ir_from_source(source: &Source) -> CompilationResult<Ir> {
    let ast = parse::parse(&source)?.0;
    lower::lower(&source, &ast)
}

macro_rules! unit_test {
    ($name:ident => |$ir:ident, $diag:ident| $body:block) => {
        #[test_that::test]
        fn $name() {
            let source = source_from_test_file(stringify!($name));
            match ir_from_source(&source) {
                Ok(($ir, $diag)) => $body,
                Err(err) => panic!(
                    "{}",
                    err.render_to_string(&&source, &DiagRenderOptions::default())
                        .expect("No error")
                ),
            }
        }
    };
}

macro_rules! snapshot_test {
    // A successful snapshot test without errors and warnings.
    ($name:ident => ok) => {
        #[test_that::test]
        fn $name() {
            let name = stringify!($name);
            let source = source_from_test_file(name);
            match ir_from_source(&source) {
                Ok((ir, diag)) => {
                    if diag.has_errors() || diag.has_warnings() {
                        panic!("{diag:?}");
                    }
                    insta::assert_snapshot!(name, lower::to_ron(&ir).expect("No error"));
                }
                Err(err) => panic!(
                    "{}",
                    err.render_to_string(&&source, &DiagRenderOptions::default())
                        .expect("No error")
                ),
            }
        }
    };

    // A successful snapshot test with errors.
    ($name:ident => error) => {
        #[test]
        fn $name() {
            let name = stringify!($name);
            let source = source_from_test_file(name);
            match ir_from_source(&source) {
                Ok((ir, diag)) => {
                    assert!(diag.has_errors());
                    insta::assert_snapshot!(name, lower::to_ron(&ir).expect("No error"));
                }
                Err(err) => panic!(
                    "{}",
                    err.render_to_string(&&source, &DiagRenderOptions::default())
                        .expect("No error")
                ),
            }
        }
    };
}

unit_test!(module => |ir, diag| {
    assert_that!(ir, matches_pattern!(ir::Source {
        *statements: len(eq(3)),
        items: matches_pattern!(ir::SourceItems {
            constants: len(eq(1)),
            inline_modules: [matches_pattern!(ir::InlineModule {
                visibility: eq(ir::Visibility::Private),
                items: matches_pattern!(ir::InlineModuleItems {
                    constants: len(eq(1)),
                    functions: [matches_pattern!(ir::Function {
                        visibility: eq(ir::Visibility::Public),
                    })],
                    workbenches: [matches_pattern!(ir::Workbench {
                        id: eq(Identifier::from("MySketch")),
                        visibility: eq(ir::Visibility::Public),
                    })]
                })
            })]
        })
    }));

    assert!(!diag.has_errors())
});

snapshot_test!(circle => ok);

#[test_that::test]
fn unexpected_statements() {
    let source = source_from_test_file("unexpected_statements");
    let parse_context = microcad_lang_parse::ParseContext::from(&source);
    let ast = Ast::parse(&parse_context).unwrap();
    let ir = lower::lower(&source, &ast);

    match ir {
        Ok((_ir, _diag)) => {
            panic!("This test is supposed to fail");
        }
        Err(diags) => {
            let observed = ExpectedDiagnostics::new(
                diags
                    .iter()
                    .map(|diag| ExpectedDiagnostic {
                        severity: diag.severity().unwrap_or_default(),
                        line: diag.src_ref.line().unwrap(),
                        code: diag.code().map(|code| code.to_string()),
                    })
                    .collect(),
            );

            println!(
                "{}",
                diags
                    .render_to_string(&&source, &DiagRenderOptions::default())
                    .unwrap()
            );

            let expected = ast::visitor::collect_expected_diagnostics(&parse_context, &ast);
            assert_that!(observed, eq(expected));
        }
    }
}

#[test_that::test]
fn serde_circle() {
    let source = source_from_test_file("circle");
    let in_ir = ir_from_source(&source).unwrap().0;

    // 1. Serialize/Deserialize
    let serialized = lower::to_ron(&in_ir).expect("Serialization failed");
    let out_ir: Ir = lower::from_ron(&*serialized).expect("Deserialization failed");

    // 2. Structural Equality
    assert_that!(
        &in_ir,
        eq(&out_ir),
        "IR structure changed after round-trip!"
    );

    // 3. String Identity
    let re_serialized = lower::to_ron(&out_ir).expect("Re-serialization failed");
    assert_that!(
        serialized,
        eq(re_serialized),
        "RON string output changed after round-trip!"
    );
}
