// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Manifest for µcad standard library in `lib.toml`

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use thiserror::Error;

/// Manifest error.
#[derive(Debug, Error)]
pub enum ManifestError {
    /// An IO error while processing the `manifest.toml` file.
    #[error("I/O error while reading manifest")]
    Io(#[from] std::io::Error),

    /// A TOML parse error.
    #[error("failed to parse manifest")]
    TomlDeserialize(#[from] toml::de::Error),

    /// A TOML write error.
    #[error("failed to write manifest")]
    TomlSerialize(#[from] toml::ser::Error),

    /// The `manifest.toml` file does not exist in the path.
    #[error("`manifest.toml` not found in {path}")]
    NotFound { path: std::path::PathBuf },
}

/// Library descriptor.
#[derive(Serialize, Deserialize)]
pub struct Library {
    /// A short description of the library.
    pub description: Option<String>,
    /// Standard library version.
    pub version: semver::Version,
    /// Authors of the library.
    pub authors: Option<Vec<String>>,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            description: Some(String::from("µcad standard library")),
            version: crate::version(),
            authors: Some(
                [
                    String::from("Patrick Hoffmann"),
                    String::from("Michael Winkelmann"),
                ]
                .into(),
            ),
        }
    }
}

/// Manifest descriptor.
#[derive(Serialize, Deserialize, Default)]
pub struct Manifest {
    /// Library descriptor.
    pub library: Library,
}

impl Manifest {
    // Load a `manifest.toml` inside a path.
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ManifestError> {
        let manifest_path = Self::manifest_path(&path);
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

    /// Save a `manifest.toml` inside a path.
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), ManifestError> {
        let s = toml::to_string(&self)?;
        let mut file = std::fs::File::create(Self::manifest_path(path))?;
        file.write(s.as_bytes())?;
        Ok(())
    }

    /// Return `manifest.toml` file path.
    pub fn manifest_path(path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        path.as_ref().join("manifest.toml")
    }
}
