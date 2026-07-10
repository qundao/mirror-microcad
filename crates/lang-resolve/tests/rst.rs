// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Artifact, Id, Identifier, SrcRef};
use microcad_lang_lower::ir::DocBlock;
use microcad_lang_resolve::{
    Rst, SymbolRef,
    rst::{
        self, ResolvedName, SymbolData, SymbolTree, UnresolvedName,
        def::{InlineModule, UnresolvedSymbolDef},
    },
};

use test_that::prelude::*;

fn inline_module(id: &str) -> rst::Symbol<rst::def::UnresolvedSymbolDef> {
    rst::Symbol {
        data: rst::SymbolData {
            visibility: rst::def::Visibility::Public,
            src_ref: SrcRef::none(),
            keyword_ref: SrcRef::none(),
            id: Id::from(id),
            doc: DocBlock::default(),
        },
        def: rst::def::UnresolvedSymbolDef::InlineModule(InlineModule),
        parent: None,
        children: Default::default(),
    }
}

fn sample_rst() -> Rst {
    let mut builder = rst::Builder::new(inline_module("root"));

    builder
        .enter(inline_module("foo"))
        .enter(inline_module("baz"))
        .add(inline_module("bam"))
        .exit() // exit baz
        .exit() // exit foo
        .add(inline_module("bar"));

    let rst = builder.build();
    println!("{rst:#?}");
    rst
}

#[test]
fn descendants_builder() {
    let tree = sample_rst();
    let root = tree.root().expect("Root should exist");

    let s = root
        .descendants()
        .map(|s| s.id().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    assert_that!(s, eq("root foo baz bam bar"));
}

#[test]
fn resolve() {
    fn resolve_id<'tree>(root: SymbolRef<'tree, rst::def::ResolvedSymbolDef>, id: &str) -> Id {
        root.resolve(id).unwrap().id().clone()
    }
    let tree = sample_rst();

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
    let mut tree = sample_rst();

    let foo = {
        let root = tree.root().unwrap();
        root.resolve("root::foo").unwrap()
    };

    tree.insert(Some(foo.handle()), sample_rst());

    let s = tree
        .root()
        .unwrap()
        .descendants()
        .map(|s| s.id().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    assert_that!(s, eq("root foo baz bam root foo baz bam bar bar"));
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
