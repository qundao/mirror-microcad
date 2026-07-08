// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::Display;
use microcad_core::hash::HashId;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::Version;

#[derive(Debug, Error, Diagnostic)]
pub enum ArtifactError {
    /// Postcard error
    #[error("Binary serialization error")]
    BinarySerialization(#[from] postcard::Error),

    /// Error during serializing to RON.
    #[error("RON serialization error")]
    RonSerialization(#[from] ron::Error),

    /// Error during deserializing from RON.
    #[error("RON serialization error")]
    RonDeserialization(#[from] ron::error::SpannedError),

    /// Wrong artifact kind
    #[error(": {expected} != {got}")]
    WrongArtifactKind {
        expected: ArtifactKind,
        got: ArtifactKind,
    },

    /// Version mismatch
    #[error("Version mismatch: {expected} != {got}")]
    VersionMismatch { expected: Version, got: Version },
}

#[derive(Debug, Display, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ArtifactKind {
    Ir,
    Ast,
    SymbolTree,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ArtifactHeader {
    pub magic: [u8; 4],
    pub version: Version,    // Increment this whenever the format changes
    pub kind: ArtifactKind,  // The artifact type (AST, IR, etc.)
    pub source_hash: HashId, // The hash of the source used (for reproducibility)
}

impl ArtifactHeader {
    pub fn new(kind: ArtifactKind, source_hash: HashId) -> ArtifactHeader {
        Self {
            magic: *b"00B5", // µ unicode
            version: Version::current(),
            kind,
            source_hash,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EnvelopeRef<'artifact, T> {
    pub header: ArtifactHeader,
    pub payload: &'artifact T,
}

#[derive(Debug, Deserialize)]
pub struct Envelope<T> {
    pub header: ArtifactHeader,
    pub payload: T,
}

impl<T> Envelope<T> {
    pub fn unwrap(self, kind: ArtifactKind) -> Result<T, ArtifactError> {
        if kind != self.header.kind {
            return Err(ArtifactError::WrongArtifactKind {
                expected: kind,
                got: self.header.kind,
            });
        }

        let version = self.header.version;
        if !version.is_compatible() {
            return Err(ArtifactError::VersionMismatch {
                expected: Version::current(),
                got: version,
            });
        }

        Ok(self.payload)
    }
}

pub trait Artifact: Sized {
    fn kind() -> ArtifactKind;

    fn source_hash(&self) -> HashId {
        Default::default()
    }

    fn envelope(&'_ self) -> EnvelopeRef<'_, Self> {
        EnvelopeRef {
            header: ArtifactHeader::new(Self::kind(), self.source_hash()),
            payload: self,
        }
    }

    /// Load the artifact from binary
    fn from_binary<'de>(bytes: &'de [u8]) -> Result<Self, ArtifactError>
    where
        Self: Deserialize<'de>,
    {
        let envelope: Envelope<Self> = postcard::from_bytes(bytes)?;
        envelope.unwrap(Self::kind())
    }

    /// Convert the artifact into binary
    fn to_binary(&self) -> Result<Vec<u8>, ArtifactError>
    where
        Self: Serialize,
    {
        Ok(postcard::to_allocvec(&self.envelope())?)
    }

    /// Convert the artifact into a Rusty Object Notation
    fn to_ron(&self) -> Result<String, ArtifactError>
    where
        Self: Serialize,
    {
        // Configure indentation, spacing, and multi-line breaks
        let config = ron::ser::PrettyConfig::default()
            .depth_limit(8)
            .indentor("    ".to_string()) // Beautiful 4-space indent
            .new_line("\n".to_string());

        Ok(ron::ser::to_string_pretty(&self.envelope(), config)?)
    }

    fn from_ron<'de>(s: impl Into<&'de str>) -> Result<Self, ArtifactError>
    where
        Self: Deserialize<'de>,
    {
        let envelope: Envelope<Self> = ron::de::from_str(s.into())?;
        envelope.unwrap(Self::kind())
    }
}
