// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Library tests

use std::str::FromStr;

use microcad_lang_base::{DiagRenderOptions, Diagnostics};
use microcad_lang_lower::ir::UnresolvedPath;
use microcad_lang_resolve::{Library, ResolveContext, SymbolNodeExt, SymbolNodeRef};

fn ctx<'lib>(lib: &'lib mut Library) -> ResolveContext<'lib> {
    let mut ctx = ResolveContext::new(lib);
    ctx.load_external_library("../../crates/std/lib/std")
        .expect("No error");
    ctx
}

fn path(name: &str) -> UnresolvedPath {
    UnresolvedPath::from_str(name).expect("A valid path")
}

#[test]
fn load_source_inline_module() {
    let mut library = Library::new();
    let mut context = ctx(&mut library);

    match context.load_file("tests/test_cases/inline_module.µcad") {
        Ok(context) => {
            let library = context.lib();
            for child in library.root().children() {
                println!("{child}");
            }
            let inline_module = library.root().find_child("inline_module").expect("A child");

            let b = library
                .look_up(inline_module, &path("b"))
                .expect("Add node");
            library
                .look_up(b, &path("::inline_module"))
                .expect("An inline module");

            let d = library
                .look_up(b, &path("::inline_module::b::d"))
                .expect("An inline module 'd'");

            let _c = library
                .look_up(b, &path("::inline_module::b::C"))
                .expect("A constant 'C'");

            // Search `b` in `d`
            let _b = library
                .look_up(d, &path("b"))
                .expect("An inline module 'b'");

            insta::assert_snapshot!("load_source_inline_module", library);
        }
        Err(err) => {
            let diagnostics = Diagnostics::from(context.diag);
            let source_cache = context.src_cache;

            panic!(
                "{diagnostics}\n{err:?}",
                diagnostics = diagnostics
                    .render_to_string(&source_cache, &DiagRenderOptions::default())
                    .unwrap()
            )
        }
    }
}

#[test]
fn load_source_file_module() {
    let mut library = Library::new();
    let mut context = ctx(&mut library);

    context
        .load_file("tests/test_cases/file_module.µcad")
        .expect("No error");

    insta::assert_snapshot!("load_source_file_module", library)
}

#[test]
fn resolve() {
    let mut library = Library::new();
    let mut context = ctx(&mut library);

    context
        .load_file("tests/test_cases/resolve.µcad")
        .expect("No error")
        .resolve();

    let symbol = library
        .look_up(library.root, &path("::resolve"))
        .map(|id| SymbolNodeRef::new(id, &library.arena))
        .expect("A node");

    insta::assert_snapshot!("resolve", &symbol)
}
