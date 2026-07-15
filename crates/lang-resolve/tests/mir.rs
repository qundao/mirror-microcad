// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Artifact, HashId, Id, SrcRef};
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

fn sample_mir() -> Mir {
    Mir {
        input_hash: HashId::default(),
        output_hash: HashId::default(),
        tree: sample_tree(),
    }
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

    assert_that!(resolve_id(root, "foo"), eq("foo"));
    assert_that!(resolve_id(root, "foo::baz"), eq("baz"));
    assert_that!(resolve_id(root, "foo::bam"), eq("bam"));
    assert_that!(resolve_id(root, "bar"), eq("bar"));

    let foo = root.resolve("foo").unwrap();
    assert_that!(resolve_id(foo, "baz"), eq("baz"));
    assert_that!(resolve_id(foo, "bam"), eq("bam"));
}

#[test]
fn insert_tree() {
    let mut tree = sample_tree();

    let foo = {
        let root = tree.root().unwrap();
        root.resolve("foo").unwrap()
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
fn scaffold_inline_module_def() {
    let source = microcad_lang_base::Source::load("tests/test_cases/inline_module.µcad")
        .expect("No errors loading source");

    let ron = std::fs::read_to_string("tests/test_cases/inline_module.µcad.ir")
        .expect("No errors reading IR");
    let ir = microcad_lang_lower::Ir::from_ron(ron.as_str()).expect("No error deserializing IR");
    let (mir, diag) = microcad_lang_resolve::scaffold(&ir, &source).expect("No error resolving IR");
    println!("{mir:#?}");
    assert!(!diag.has_errors());

    let root = mir.tree.root().unwrap();

    let s = root
        .descendants()
        .filter_map(|s| s.id().map(|id| id.to_string()))
        .collect::<Vec<_>>()
        .join(" ");

    assert_that!(s, eq("root A b C d E f G"));

    assert!(root.resolve("A").is_some());
    assert!(root.resolve("b::C").is_some());
    assert!(root.resolve("b::d::E").is_some());
    assert!(root.resolve("b::f::G").is_some()); // Should be unreachable*/
}

#[test]
fn mir_binary() {
    let mir = sample_mir();
    let encoded_mir: Vec<u8> = mir.to_binary().expect("No error");
    let decoded_mir = Mir::from_binary(&encoded_mir).expect("Failed to Deserialize");

    assert_that!(mir, eq(decoded_mir));
}
