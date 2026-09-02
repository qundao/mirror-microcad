// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display snapshot tests for IR

use microcad_lang_base::DiagRenderOptions;
mod common;

macro_rules! snapshot_test_display {
    // A successful snapshot test without errors and warnings.
    ($name:ident) => {
        #[test_that::test]
        fn $name() {
            let name = stringify!($name);
            let source = common::source_from_test_file(name);
            match common::ir_from_source(&source) {
                Ok((ir, diag)) => {
                    if diag.has_errors() || diag.has_warnings() {
                        panic!("{diag:?}");
                    }
                    insta::assert_snapshot!(name, ir);
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

snapshot_test_display!(circle);
snapshot_test_display!(builtin);
