// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{GetSourceByHash, HashId, HashMap, LibraryId, Source};

use crate::{Library, ResolveError, ResolveResult, SourceUnit, error::ResolveErrorKind};

#[derive(Debug, Default)]
pub struct LibraryCache {
    /// Global lookup for all loaded/compiled libraries
    libraries: HashMap<LibraryId, Library>,
}

impl LibraryCache {
    pub fn insert(&mut self, library: Library) -> ResolveResult<LibraryId> {
        let id = library.id();
        match id.is_root() {
            false => {
                self.libraries.insert(id, library);
                Ok(id)
            }
            true => {
                todo!("Cannot insert root library with ID")
            }
        }
    }

    pub fn get(&self, id: &LibraryId) -> Option<&Library> {
        self.libraries.get(&id)
    }
}

#[derive(Debug, Default)]
pub struct SourceCache {
    // Source by hash
    by_hash: HashMap<HashId, SourceUnit>,
    // Source by paths
    by_path: HashMap<std::path::PathBuf, HashId>,
}

impl GetSourceByHash for SourceCache {
    fn get_source_by_hash(&'_ self, hash: HashId) -> Option<&Source> {
        self.by_hash
            .get(&hash)
            .map(|source_unit| source_unit.source())
    }
}

impl SourceCache {
    pub fn insert(&mut self, source: impl Into<Source>) -> ResolveResult<HashId> {
        let source = source.into();

        if let Some(source_unit) = self.get(source.hash_id()) {
            log::debug!("Source unit '{source_unit}' is already loaded");
            return Ok(source.hash_id());
        }

        let hash_id = source.hash_id();
        match source.path() {
            Some(path) => {
                self.by_hash.insert(hash_id, SourceUnit::load(&path)?);
                self.by_path.insert(path, hash_id);
                Ok(hash_id)
            }
            None => Err(ResolveError::new(ResolveErrorKind::SourceHasNoPath(
                source.to_string(),
            ))),
        }
    }

    pub fn get(&self, id: HashId) -> Option<&SourceUnit> {
        self.by_hash.get(&id.into())
    }
}
