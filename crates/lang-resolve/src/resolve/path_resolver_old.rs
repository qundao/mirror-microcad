// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{GetSourceByHash, HashId, Identifier, Source, SrcReferrer};

use crate::resolve::ResolveError;

pub trait PathResolver: GetSourceByHash {
    fn source(&self, hash_id: HashId) -> Result<&Source, ResolveError> {
        self.get_source_by_hash(hash_id)
            .ok_or(ResolveError::NoSourceWithHash(hash_id))
    }

    fn source_path(&self, hash_id: HashId) -> Result<std::path::PathBuf, ResolveError> {
        let source = self.source(hash_id)?;

        match source.path() {
            Some(path) => Ok(path),
            None => Err(ResolveError::SourceHasNoPath {
                loc: source.location.clone(),
                src_ref: source.src_ref(),
            }),
        }
    }

    fn module_name_from_source(&self, hash_id: HashId) -> Result<Identifier, ResolveError> {
        let source_path = self.source_path(hash_id)?;

        // Extract `foo` from `file/to/foo.mu`
        let stem = source_path.file_stem().unwrap().to_str().unwrap(); // TODO Remove unwrap here.
        Ok(Identifier::from(stem))
    }

    fn file_module_path(
        &self,
        hash_id: HashId,
        file_module_name: &Identifier,
    ) -> Result<std::path::PathBuf, ResolveError> {
        Ok(self
            .source_path(hash_id)
            .map(|path| path.join(file_module_name.id().to_string()))?)
    }
}

pub struct DefaultPathResolver<'source> {
    source: &'source Source,
}

impl<'source> From<&'source Source> for DefaultPathResolver<'source> {
    fn from(source: &'source Source) -> Self {
        Self { source }
    }
}

impl<'source> GetSourceByHash for DefaultPathResolver<'source> {
    fn get_source_by_hash(&'_ self, hash: HashId) -> Option<&Source> {
        self.source.get_source_by_hash(hash)
    }
}

impl<'source> PathResolver for DefaultPathResolver<'source> {}
