// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse syntax elements tests

use insta::assert_debug_snapshot;
use microcad_lang_parse::{Parse, ParseContext, ast};
use test_case::test_case;

#[test_case("int", "1")]
#[test_case("length", "42mm")]
#[test_case("bool", "true")]
fn test_literal(name: &str, input: &str) {
    let context = ParseContext::new(input);
    assert_debug_snapshot!(format!("literal_{name}"), ast::Literal::parse(&context));
}
