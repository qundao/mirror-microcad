// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI install command.

use std::path::PathBuf;

use rust_embed::RustEmbed;
use thiserror::Error;

use crate::manifest::{Manifest, ManifestError};

mod manifest;

/// Manifest error.
#[derive(Debug, Error)]
pub enum StdLibError {
    /// An error while processing the `manifest.toml` file.
    #[error("An error while processing manifest file: {0}")]
    ManifestError(#[from] manifest::ManifestError),

    /// Error during install or uninstall.
    #[error("An error during installation: {0}")]
    InstallError(#[from] std::io::Error),
}

/// The µcad standard library asset.
#[derive(RustEmbed)]
#[folder = "lib/std"]
pub struct Lib;

/// An instance of the standard library.
pub struct StdLib {
    /// Path of the library which contains `mod.µcad` and `manifest.toml`.
    pub path: std::path::PathBuf,
    /// The parsed manifest.
    pub manifest: manifest::Manifest,
}

impl StdLib {
    /// Create a new standard library instance from a path.
    ///
    /// Installs the standard library, if it is not installed.
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self, StdLibError> {
        let path = PathBuf::from(path.as_ref());

        let manifest = match manifest::Manifest::load(&path) {
            Ok(manifest) => manifest,
            // Try to install the standard library, in case the `manifold.toml`` has not been found.
            Err(ManifestError::NotFound { path }) => Self::install(&path)?,
            Err(err) => return Err(err.into()),
        };

        let manifest = if manifest.library.version != Self::crate_version() {
            eprintln!(
                "µcad standard library version mismatch: {} != {}",
                manifest.library.version,
                Self::crate_version()
            );

            // Handle version mismatch, force re-install
            Self::reinstall(true)?
        } else {
            manifest
        };

        Ok(Self { path, manifest })
    }

    /// Return the version number of this crate.
    pub fn crate_version() -> semver::Version {
        use std::str::FromStr;
        semver::Version::from_str(env!("CARGO_PKG_VERSION")).expect("Valid version")
    }

    /// Try to reinstall into default path.
    pub fn reinstall(force: bool) -> Result<Manifest, StdLibError> {
        let path = Self::default_path();
        if force {
            Self::uninstall(&path)?;
        }

        Self::install(path)
    }

    /// Install the standard library into the standard library path and return its manifest.
    fn install(path: impl AsRef<std::path::Path>) -> Result<manifest::Manifest, StdLibError> {
        let path = path.as_ref();
        eprintln!(
            "Installing µcad standard library {} into {:?}...",
            Self::crate_version(),
            path
        );

        std::fs::create_dir_all(path)?;

        // Extract all embedded files.
        Lib::iter().try_for_each(|file| {
            let file_path = path.join(file.as_ref());
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(
                file_path,
                Lib::get(file.as_ref())
                    .expect("embedded folder 'lib' not found")
                    .data,
            )
        })?;

        eprintln!("Successfully installed µcad standard library.");

        Ok(manifest::Manifest::load(path)?)
    }

    /// Uninstall the standard library from the standard library path.
    fn uninstall(path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let path = path.as_ref();

        // We cannot uninstall in debug mode.
        #[cfg(debug_assertions)]
        eprintln!("µcad standard library ({path:?}) cannot to be uninstalled in debug mode.");
        return Ok(());

        #[cfg(not(debug_assertions))]
        {
            if !path.exists() {
                eprintln!(
                    "µcad standard library not found in {:?}. Nothing to uninstall.",
                    path
                );
                return Ok(());
            }

            eprintln!("Removing µcad standard library from {:?}...", path);

            std::fs::remove_dir_all(path)?;

            eprintln!("Successfully uninstalled µcad standard library.");

            Ok(())
        }
    }

    /// Global library search path + `./std`.
    pub fn default_path() -> std::path::PathBuf {
        global_library_search_path().join("std")
    }
}

pub fn global_library_search_path() -> std::path::PathBuf {
    #[cfg(not(debug_assertions))]
    return dirs::config_dir()
        .expect("config directory")
        .join("microcad")
        .join("lib");

    #[cfg(debug_assertions)]
    return std::path::PathBuf::from("./crates/std/lib");
}
