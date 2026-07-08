// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::SymbolTree;

use microcad_lang_base::Version;
use thiserror::Error;

pub struct TreeStorage;

#[derive(Debug, Error)]
pub enum PersistenceError {
    /// Postcard error
    #[error("Serialization error")]
    Serialization(#[from] postcard::Error),

    /// Version mismatch
    #[error("Version mismatch: {expected} != {got}")]
    VersionMismatch { expected: Version, got: Version },
}

impl TreeStorage {
    /// Saves the tree to a byte buffer using postcard
    pub fn save_bin(tree: &SymbolTree) -> Result<Vec<u8>, PersistenceError> {
        Ok(postcard::to_allocvec(tree)?)
    }

    /// Loads the tree from a byte buffer using postcard
    pub fn load_bin(bytes: &[u8]) -> Result<SymbolTree, PersistenceError> {
        let tree: SymbolTree = postcard::from_bytes(bytes)?;

        if tree.metadata.version.is_compatible() {
            Ok(tree)
        } else {
            Err(PersistenceError::VersionMismatch {
                expected: Version::current(),
                got: tree.metadata.version,
            })
        }
    }
}
