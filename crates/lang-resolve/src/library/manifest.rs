// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Data structure for the package manifest, saved as `mu.toml`.

use microcad_lang_base::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Manifest error.
#[cfg(feature = "io")]
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

    /// The `mu.toml` file does not exist in the path.
    #[error("`mu.toml` not found in {path}")]
    NotFound { path: std::path::PathBuf },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub library: LibrarySection,
    pub dependencies: HashMap<String, Dependency>,
}

/// `package` descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySection {
    /// Mandatory package name.
    pub name: String,
    /// A short description of the library.
    pub description: Option<String>,
    /// Mandatory package version.
    pub version: semver::Version,
    /// Authors of the library.
    pub authors: Option<Vec<String>>,
    /// Documentation URL
    pub documentation: Option<Url>,
    /// License
    pub license: Option<String>,
    /// Do not load the standard library by default.
    pub no_std: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub path: Option<std::path::PathBuf>,
}

#[cfg(feature = "io")]
impl Manifest {
    pub const MANIFEST_FILE_NAME: &str = "mu.toml";

    // Load a `manifest.toml` inside a path.
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ManifestError> {
        let manifest_path = Self::manifest_path(&path);
        if !manifest_path.exists() || !manifest_path.is_file() {
            return Err(ManifestError::NotFound {
                path: std::path::PathBuf::from(path.as_ref()),
            });
        }

        let buf = std::fs::read_to_string(manifest_path)?;
        Ok(toml::from_str(&buf)?)
    }

    /// Save a `manifest.toml` inside a path.
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), ManifestError> {
        use std::io::Write;
        let s = toml::to_string(&self)?;
        let mut file = std::fs::File::create(Self::manifest_path(path))?;
        file.write_all(s.as_bytes())?;
        Ok(())
    }

    /// Return `mu.toml` file path.
    pub fn manifest_path(path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        let path = path.as_ref();
        match path.file_name() {
            Some(filename) if filename == Self::MANIFEST_FILE_NAME => path.to_path_buf(),
            _ => path.join(Self::MANIFEST_FILE_NAME),
        }
    }
}
