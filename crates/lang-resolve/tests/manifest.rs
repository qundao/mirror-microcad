// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_resolve::library::Manifest;

mod common;

use crate::common::test_file_path;

/// Load manifest file.
#[test]
fn load_mu_toml() {
    let manifest = Manifest::load(test_file_path("mu.toml")).expect("No error");

    insta::assert_debug_snapshot!("manifest", manifest)
}
