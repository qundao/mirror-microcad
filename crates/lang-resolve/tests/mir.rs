// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Artifact, Id, SrcRef};
use microcad_lang_lower::ir::DocBlock;
use microcad_lang_resolve::{Mir, mir, scaffold};

use test_that::prelude::*;

fn inline_module(id: &str) -> mir::UnresolvedSymbol {
    mir::UnresolvedSymbol {
        meta_data: mir::SymbolMetadata {
            visibility: mir::Visibility::Public,
            src_ref: SrcRef::none(),
            keyword_src_ref: SrcRef::none(),
            id: Some(mir::Identifier::from(id)),
            doc: mir::DocBlock::default(),
        },
        def: mir::UnresolvedSymbolDef::InlineModule,
        parent: None,
        children: Default::default(),
    }
}

fn sample_tree() -> mir::UnresolvedSymbolTree {
    let mut builder = scaffold::TreeBuilder::new(inline_module("root"));

    builder
        .enter(inline_module("foo"))
        .enter(inline_module("baz"))
        .add(inline_module("bam"))
        .exit() // exit baz
        .exit() // exit foo
        .add(inline_module("bar"));

    let tree = builder.build();
    println!("{tree:#?}");
    tree
}

#[test]
fn descendants_builder() {
    let tree = sample_tree();
    let root = tree.root().expect("Root should exist");

    let s = root
        .descendants()
        .filter_map(|s| s.id().map(|id| id.to_string()))
        .collect::<Vec<_>>()
        .join(" ");

    assert_that!(s, eq("root foo baz bam bar"));
}

#[test]
fn resolve() {
    fn resolve_id<'mir>(root: mir::UnresolvedSymbolRef<'mir>, id: &str) -> Id {
        root.resolve(id).unwrap().id().unwrap().id().clone()
    }
    let tree = sample_tree();

    let root = tree.root().unwrap();

    assert_that!(resolve_id(root, "root::foo"), eq("foo"));
    assert_that!(resolve_id(root, "root::foo::baz"), eq("baz"));
    assert_that!(resolve_id(root, "root::foo::bam"), eq("bam"));
    assert_that!(resolve_id(root, "root::bar"), eq("bar"));

    let foo = root.resolve("root::foo").unwrap();
    assert_that!(resolve_id(foo, "foo"), eq("foo"));
    assert_that!(resolve_id(foo, "foo::baz"), eq("baz"));
    assert_that!(resolve_id(foo, "foo::bam"), eq("bam"));
    assert_that!(resolve_id(foo, "root"), eq("root"));
}

#[test]
fn insert_tree() {
    let mut tree = sample_tree();

    let foo = {
        let root = tree.root().unwrap();
        root.resolve("root::foo").unwrap()
    };

    tree.insert(Some(foo.handle()), sample_tree());

    let s = tree
        .root()
        .unwrap()
        .descendants()
        .filter_map(|s| s.id().map(|id| id.to_string()))
        .collect::<Vec<_>>()
        .join(" ");

    assert_that!(s, eq("root foo baz bam root foo baz bam bar bar"));
}

#[test]
fn resolve_inline_module_def() {
    let source = microcad_lang_base::Source::load("tests/test_cases/inline_module.µcad")
        .expect("No errors loading source");

    /*
    let ron = std::fs::read_to_string("tests/test_cases/inline_module.µcad.ir")
        .expect("No errors reading IR");
    let ir = microcad_lang_lower::Ir::from_ron(ron.as_str()).expect("No error deserializing IR");
    let (rst, diag) = microcad_lang_resolve::resolve(&source, &ir).expect("No error resolving IR");
    println!("{rst:#?}");
    assert!(!diag.has_errors());

    let root = rst.root().unwrap();

    assert!(root.resolve("root::A").is_some());
    assert!(root.resolve("root::b::C").is_some());
    assert!(root.resolve("root::b::d::E").is_some());
    assert!(root.resolve("root::b::f::G").is_some()); // Show be unreachable
    */
}

/*
#[test]
fn binary() {
    let tree = sample_unresolved_tree();
    let encoded: Vec<u8> = tree.to_binary().expect("No error");
    let decoded_tree = Rst::from_binary(&encoded).expect("Failed to Deserialize");

    assert_that!(tree, eq(decoded_tree));
}
*/
