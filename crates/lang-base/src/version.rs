// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// The version string from cargo.
pub const MICROCAD_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use semver::Version;

/// Storage for µcad language version
#[derive(Debug, Display, PartialEq, Serialize, Deserialize)]
#[display("{_0}")]
pub struct LanguageVersion(Version);

impl LanguageVersion {
    /// The current µcad language version
    pub fn current() -> Self {
        Self(Version::parse(MICROCAD_VERSION).expect("A valid cargo package version"))
    }

    /// Checks if a version string is compatible with the current µcad compiler
    pub fn is_compatible(&self) -> bool {
        use semver::VersionReq;

        // Use caret requirements: ^0.1.0 allows 0.1.x but not 0.2.0
        let req = VersionReq::parse(&format!("^{}", MICROCAD_VERSION)).unwrap();

        req.matches(&self.0)
    }
}

impl From<LanguageVersion> for Version {
    fn from(value: LanguageVersion) -> Self {
        value.0
    }
}

impl std::str::FromStr for LanguageVersion {
    type Err = semver::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(semver::Version::parse(s)?))
    }
}

/// Represents the stability level of a symbol in µcad.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stability {
    Experimental,
    Stable,
    Deprecated {
        since: Version,
        note: Option<String>,
        removal_in: Option<Version>,
    },
}

/// Version annotation for a symbol in µcad.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionAnnotation {
    /// The version when the symbol was introduced to the DSL.
    pub introduced: Version,

    /// Current stability state of the symbol.
    pub stability: Stability,
}

impl VersionAnnotation {
    /// Creates a standard stable version annotation.
    pub fn stable(introduced: Version) -> Self {
        Self {
            introduced,
            stability: Stability::Stable,
        }
    }

    /// Checks if the symbol is available at a given target DSL version.
    pub fn is_available_at(&self, target_version: &Version) -> bool {
        target_version >= &self.introduced
    }

    /// Checks if the symbol is deprecated at a given target DSL version.
    pub fn is_deprecated_at(&self, target_version: &Version) -> bool {
        match &self.stability {
            Stability::Deprecated { since, .. } => target_version >= since,
            _ => false,
        }
    }
}

impl Default for VersionAnnotation {
    fn default() -> Self {
        Self {
            introduced: LanguageVersion::current().into(),
            stability: Stability::Stable,
        }
    }
}
