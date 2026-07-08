// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// The version string from cargo.
pub const MICROCAD_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Storage for µcad language version
#[derive(Debug, Display, PartialEq, Serialize, Deserialize)]
#[display("{_0}")]
pub struct Version(semver::Version);

impl Version {
    /// The current µcad language version
    pub fn current() -> Self {
        Self(semver::Version::parse(MICROCAD_VERSION).expect("A valid cargo package version"))
    }

    /// Checks if a version string is compatible with the current µcad compiler
    pub fn is_compatible(&self) -> bool {
        use semver::VersionReq;

        // Use caret requirements: ^0.1.0 allows 0.1.x but not 0.2.0
        let req = VersionReq::parse(&format!("^{}", MICROCAD_VERSION)).unwrap();

        req.matches(&self.0)
    }
}

impl std::str::FromStr for Version {
    type Err = semver::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(semver::Version::parse(s)?))
    }
}
