use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_resolve::symbol::{self, TreeStorage};

use test_that::prelude::*;

fn inline_module(id: &str) -> symbol::SymbolData {
    symbol::SymbolData {
        id: Identifier::from(id),
        attr: symbol::SymbolAttributes::default(),
        def: symbol::def::InlineModule.into(),
        visibility: symbol::def::Visibility::Public,
        src_ref: SrcRef::none(),
        keyword_ref: SrcRef::none(),
    }
}

fn sample_tree() -> symbol::SymbolTree {
    let mut builder = symbol::SymbolTreeBuilder::new(inline_module("root"));

    builder
        .enter(inline_module("foo"))
        .enter(inline_module("baz"))
        .add(inline_module("bam"))
        .exit() // exit baz
        .exit() // exit foo
        .add(inline_module("bar"));

    builder.build()
}

#[test]
fn descendants_builder() {
    let tree = sample_tree();
    let root = tree.root().expect("Root should exist");

    let s = root
        .descendants()
        .map(|s| s.id().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    assert_eq!(s, "root foo baz bam bar");
}

#[test_that::test]
fn tree_storage_bin() {
    let tree = sample_tree();
    let encoded: Vec<u8> = TreeStorage::save_bin(&tree).expect("Failed to serialize tree");
    let decoded_tree = TreeStorage::load_bin(&encoded).expect("Failed to Deserialize");

    assert_that!(tree, eq(decoded_tree));
}
