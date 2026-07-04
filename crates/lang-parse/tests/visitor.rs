// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use insta::assert_debug_snapshot;
use microcad_lang_parse::ast::visitor::{Visit, Visitor};
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
fn test_comment_collector(name: &str, input: &str) {
    let ast = microcad_lang_parse::parse(&microcad_lang_base::Source::from(input))
        .expect("No error")
        .0;
    let mut comment_collector = microcad_lang_parse::ast::visitor::CommentCollector::default();
    ast.visit(&mut comment_collector);

    assert_debug_snapshot!(format!("visitor_{name}"), comment_collector);
}
