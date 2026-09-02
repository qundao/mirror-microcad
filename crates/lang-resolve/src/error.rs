// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A resolve error

use microcad_lang_base::{HashId, SrcRef, SrcReferrer, element::Case};

use microcad_lang_types::Type;
use miette::Diagnostic;
use thiserror::Error;

use crate::locate::LocateError;

#[derive(Debug, Error, Diagnostic)]
pub enum ResolveError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    Locate(#[from] LocateError),

    #[error("Wrong case")]
    #[diagnostic(severity = "warning")]
    WrongCase {
        expected: Case,
        actual: Case,
        #[label]
        src_ref: SrcRef,
    },
    #[error("Type mismatch: {specified} != {actual}")]
    TypeMismatch {
        specified: Type,
        #[label("Specified type")]
        specified_src_ref: SrcRef,
        actual: Type,
        #[label("Actual type")]
        actual_src_ref: SrcRef,
    },
    #[error("No source with hash: {0}")]
    NoSourceWithHash(HashId),
}

pub type ResolveResult<T> = Result<T, ResolveError>;

impl SrcReferrer for ResolveError {
    fn src_ref(&self) -> SrcRef {
        match self {
            ResolveError::WrongCase { src_ref, .. } => *src_ref,
            ResolveError::TypeMismatch {
                specified_src_ref, ..
            } => *specified_src_ref,
            _ => SrcRef::none(),
        }
    }
}
