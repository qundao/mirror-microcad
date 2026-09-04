// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Artifact, DiagRenderOptions};
use microcad_lang_lower::{self as lower, Ir};
use microcad_lang_parse::{
    self as parse, Ast, Parse,
    ast::visitor::{ExpectedDiagnostic, ExpectedDiagnostics},
};

mod common;

use parse::ast;

use test_that::prelude::*;

macro_rules! _unit_test {
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
            let source = common::source_from_test_file(name);
            match common::ir_from_source(&source) {
                Ok((ir, errors)) => {
                    let diags = microcad_lang_base::Diagnostics::from(errors);
                    if diags.has_errors() || diags.has_warnings() {
                        panic!("{diags:?}");
                    }
                    insta::assert_snapshot!(name, ir.to_ron().expect("No error"));
                }
                Err(errors) => {
                    let diags = microcad_lang_base::Diagnostics::from(errors);
                    panic!(
                        "{}",
                        diags
                            .render_to_string(&&source, &DiagRenderOptions::default())
                            .expect("No error")
                    )
                }
            }
        }
    };

    // A successful snapshot test with (expected) errors.
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
            let source = common::source_from_test_file(filename);
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
                Err(errors) => {
                    let diags = microcad_lang_base::Diagnostics::from(errors);
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

snapshot_test!(circle => ok);
snapshot_test!(builtin => ok);

test_diagnostic!(unexpected_statements);
test_diagnostic!(init);
test_diagnostic!(init_statement);

/// Test serialization
#[test_that::test]
fn serde_circle() {
    let source = common::source_from_test_file("circle");
    let in_ir = common::ir_from_source(&source).unwrap().0;

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
