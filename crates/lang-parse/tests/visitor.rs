// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for visitor pattern.

use insta::assert_debug_snapshot;
use microcad_lang_parse::{
    Ast, Parse, ParseContext,
    ast::visitor::{CommentCollector, ExpectedDiagnostic, ExpectedDiagnosticsCollector, Visit},
};
use miette::Severity;
use test_case::test_case;

#[test_case(
    "comment collector",
    "
    // A
    foo(
        1, // B
        2
    );
    // C
    "
)]
fn comment_collector(name: &str, input: &str) {
    let ast = microcad_lang_parse::parse(&microcad_lang_base::Source::from(input))
        .expect("No error")
        .0;
    let mut comment_collector = CommentCollector::default();
    let _ = ast.tree().visit(&mut comment_collector);

    assert_debug_snapshot!(format!("visitor_{name}"), comment_collector);
}

#[test]
fn expected_diagnostic_from_comment() {
    let line = 10;
    let diag1 = ExpectedDiagnostic::from_comment("// error: UnexpectedToken", line).expect("Some");

    assert_eq!(diag1.severity, Severity::Error);
    assert_eq!(diag1.code.unwrap(), "UnexpectedToken");

    let diag2 = ExpectedDiagnostic::from_comment("// warning", line).expect("Some");
    assert_eq!(diag2.severity, Severity::Warning);
    assert!(diag2.code.is_none());

    assert!(ExpectedDiagnostic::from_comment("// Regular comment", line).is_none());
}

#[test_case(
    "expected diagnostic collector",
    "
    foo(1, 2); // error: SomeErrorCode
    // warning: SomeCode
    "
)]
fn expected_diagnostics_collector(name: &str, input: &str) {
    let source = microcad_lang_base::Source::from(input);
    let context = ParseContext::from(&source);
    let ast = Ast::parse(&context).expect("No parse errors");

    let mut collector = ExpectedDiagnosticsCollector::new(&context);
    let _ = ast.tree().visit(&mut collector);

    let diags = collector.diagnostics();

    assert_debug_snapshot!(format!("visitor_{name}"), diags);
}
