// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolver.

use crate::library::Manifest;

use crate::ResolveResult;

/// The interface for an resolver
pub trait Resolver {
    /// Return external search paths
    fn external_search_paths(&self) -> Vec<std::path::PathBuf>;

    /// Return the source path of the file to be resolved
    fn source_path(&self) -> std::path::PathBuf;

    fn workspace_path(&self) -> std::path::PathBuf;

    fn lib_file_path(&self) -> Option<std::path::PathBuf>;

    /// Loads the `mu.toml` manifest file as TOML from workspace root.
    fn load_manifest(&mut self) -> ResolveResult<Option<Manifest>>;
}
