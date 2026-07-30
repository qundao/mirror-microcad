// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Package data structure

#[derive(Debug)]
pub struct Dependencies {
    packages: HashMap<Id, Arc<Package>>,
}

pub struct PackageMetadata {
    version: semver::Version,
    authors: Vec<Author>,
}

#[derive(Debug)]
pub struct Package {
    metadata: PackageMetadata,
    dependencies: Dependencies,
    library: Library,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    hash_id: HashId,
    /// Resolved symbol tree.
    rst: Rst,
}
