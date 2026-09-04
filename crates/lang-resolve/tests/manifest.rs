// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_resolve::library::{Dependency, LibrarySection, Manifest};

mod common;

use test_that::prelude::*;

use crate::common::test_file_path;

/// Load manifest file.
#[test]
fn load_mu_toml() {
    let manifest = Manifest::load(test_file_path("mu.toml")).expect("No error");

    assert_that!(
        manifest,
        matches_pattern!(Manifest {
            library: matches_pattern!(LibrarySection {
                name: eq("test-lib".to_string()),
                description: some(eq("A test library".to_string())),
                version: eq(semver::Version::parse("0.1.0").unwrap()),
                authors: some(eq(vec!["John Doe".to_string(), "Jane Doe".to_string()])),
                documentation: some(eq("https://mu.xyz/test_lib".parse().unwrap())),
                license: some(eq("GPLv3".to_string())),
                no_std: some(eq(true))
            }),
            dependencies: has_entry(
                String::from("a"),
                matches_pattern!(Dependency {
                    path: some(eq(std::path::PathBuf::from("../std"))),
                })
            ),
        })
    );
}
