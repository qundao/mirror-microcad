// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{
    Artifact, CompilationResult, DiagRenderOptions, MICROCAD_EXTENSION, Source,
};
use microcad_lang_lower::{self as lower, Desugar, Ir, LowerContext, LowerResult, ir};
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
    let ast = parse::parse(source)?.0;
    let mut context = LowerContext::from(source);
    lower::lower(&mut context, &ast)
}

fn desugar(source: &Source) -> LowerResult<ir::desugared::Source> {
    let ast = parse::parse(source).expect("No parse error").0;
    let mut context = LowerContext::from(source);
    ir::desugared::Source::desugar(ast.tree(), &mut context)
}

macro_rules! desugar_unit_test {
    ($name:ident => |$ir:ident| $body:block) => {
        #[test_that::test]
        fn $name() {
            let source = source_from_test_file(stringify!($name));
            desugar(&source).expect("No error")
        }
    };
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
                    insta::assert_snapshot!(name, ir.to_ron().expect("No error"));
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
                    insta::assert_snapshot!(name, ir.to_ron().expect("No error"));
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

macro_rules! test_diagnostic {
    ($name:ident) => {
        #[test]
        fn $name() {
            let filename = stringify!($name);
            let source = source_from_test_file(filename);
            let parse_context = microcad_lang_parse::ParseContext::from(&source);
            let ast = Ast::parse(&parse_context).unwrap();
            let mut context = microcad_lang_lower::LowerContext::new(&source);
            let ir = lower::lower(&mut context, &ast);
            let expected = ast::visitor::collect_expected_diagnostics(&parse_context, &ast);

            match ir {
                Ok(_) => {
                    if !expected.is_empty() {
                        panic!("Test '{}' was expected to fail, but succeeded.", filename);
                    }
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

                    let expected = ast::visitor::collect_expected_diagnostics(&parse_context, &ast);

                    // Display diagnostics on failure
                    assert_that!(
                        observed,
                        eq(expected),
                        "{}",
                        diags
                            .render_to_string(&&source, &Default::default())
                            .unwrap(),
                    );
                }
            }
        }
    };
}

desugar_unit_test!(module => |ir| {
    assert_that!(ir.tree, matches_pattern!(ir::Source {
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

});

desugar_unit_test!(inline_module => |ir| {
    assert_that!(ir.tree, matches_pattern!(ir::Source {
        statements: empty(),
        items: matches_pattern!(ir::SourceItems {
            constants: len(eq(1)),
            inline_modules: [matches_pattern!(ir::InlineModule {
                visibility: eq(ir::Visibility::Public),
                id: eq(Identifier::from("b")),
                items: matches_pattern!(ir::InlineModuleItems {
                    constants: [matches_pattern!(
                        ir::Constant {
                            id: eq(ir::Identifier::from("C")),
                            visibility: eq(ir::Visibility::Public),
                        }
                    )],
                    modules: [
                        matches_pattern!(ir::InlineModule {
                            id: eq(Identifier::from("d")),
                            visibility: eq(ir::Visibility::Public),
                            items: matches_pattern!(ir::InlineModuleItems {
                                constants: [matches_pattern!(
                                    ir::Constant {
                                        id: eq(Identifier::from("E")),
                                        visibility: eq(ir::Visibility::Private),
                                    }
                                )],
                            }),
                        }),
                        matches_pattern!(ir::InlineModule {
                            id: eq(Identifier::from("f")),
                            items: matches_pattern!(ir::InlineModuleItems {
                                constants: [matches_pattern!(
                                    ir::Constant {
                                        id: eq(Identifier::from("G")),
                                        visibility: eq(ir::Visibility::Private),
                                    }
                                )],
                            }),

                        })
                    ],
                })
            })]
        })
    }));

    assert!(!diag.has_errors())
});

snapshot_test!(circle => ok);
//snapshot_test!(builtin => ok);

test_diagnostic!(unexpected_statements);
test_diagnostic!(init);

/// Test serialization
#[test_that::test]
fn serde_circle() {
    let source = source_from_test_file("circle");
    let in_ir = ir_from_source(&source).unwrap().0;

    // 1. Serialize/Deserialize
    let serialized = in_ir.to_ron().expect("No error");
    let out_ir = Ir::from_ron(&*serialized).expect("Deserialization failed");

    // 2. Structural Equality
    assert_that!(
        &in_ir,
        eq(&out_ir),
        "IR structure changed after round-trip!"
    );

    // 3. String Identity
    let re_serialized = out_ir.to_ron().expect("Re-serialization failed");
    assert_that!(
        serialized,
        eq(re_serialized),
        "RON string output changed after round-trip!"
    );
}
