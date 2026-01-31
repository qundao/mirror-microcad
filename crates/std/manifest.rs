// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Manifest for µcad standard library in `lib.toml`

use serde::Deserialize;
use std::io::Read;
use thiserror::Error;

/// Manifest error.
#[derive(Debug, Error)]
pub enum ManifestError {
    /// An IO error while processing the `manifest.toml` file.
    #[error("I/O error while reading manifest")]
    Io(#[from] std::io::Error),

    /// A parse error.
    #[error("failed to parse manifest TOML")]
    Toml(#[from] toml::de::Error),

    /// The `manifest.toml` file does not exist in the path.
    #[error("`manifest.toml` not found in {path}")]
    NotFound { path: std::path::PathBuf },
}

/// Library descriptor.
#[derive(Deserialize)]
pub struct Library {
    /// A short description of the library.
    pub description: Option<String>,
    /// Standard library version.
    pub version: semver::Version,
    /// Authors of the library.
    pub authors: Option<Vec<String>>,
}

/// Manifest descriptor.
#[derive(Deserialize)]
pub struct Manifest {
    pub library: Library,
}

impl Manifest {
    // Load a `manifest.toml` inside a path.
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ManifestError> {
        let manifest_path = path.as_ref().join("manifest.toml");
        if !manifest_path.exists() || !manifest_path.is_file() {
            return Err(ManifestError::NotFound {
                path: std::path::PathBuf::from(path.as_ref()),
            });
        }

        let mut file = std::fs::File::open(manifest_path)?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(toml::from_str(&buf)?)
    }
}
