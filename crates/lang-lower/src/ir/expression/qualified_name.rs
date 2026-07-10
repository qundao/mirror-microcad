// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::Deref;

use microcad_lang_base::{Identifier, Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::SrcReferrer;
use miette::SourceSpan;

use serde::{Deserialize, Serialize};

/// A *qualified name* consists of a list of *identifiers*, separated by `::`,
/// e.g. `a::b::c`
#[derive(Default, Clone, Debug, Hash, PartialEq, Deref, SrcReferrer, Serialize, Deserialize)]
pub struct QualifiedName(Refer<Box<[Identifier]>>);

impl QualifiedName {
    /// Create [`QualifiedName`] from [`Identifier`]s.
    ///
    /// - `ids`: *Identifiers* that concatenate to the *qualified name*.
    /// - `src_ref`: Reference for the whole name.
    pub fn new(ids: Vec<Identifier>, src_ref: SrcRef) -> Self {
        Self(Refer::new(ids.into_boxed_slice(), src_ref))
    }

    /// Tells if self is in a specified module
    pub fn is_within(&self, module: &QualifiedName) -> bool {
        self.starts_with(module)
    }
}

impl crate::SingleIdentifier for QualifiedName {
    fn single_identifier(&self) -> Option<&Identifier> {
        if self.is_single_identifier() {
            self.0.first()
        } else {
            None
        }
    }

    fn is_single_identifier(&self) -> bool {
        self.0.len() == 1
    }
}

impl From<QualifiedName> for SourceSpan {
    fn from(value: QualifiedName) -> Self {
        value.src_ref().into()
    }
}

impl From<Identifier> for QualifiedName {
    fn from(id: Identifier) -> Self {
        let src_ref = id.src_ref();
        Self::new(vec![id], src_ref)
    }
}

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "NO NAME")
        } else {
            write!(
                f,
                "{}",
                self.iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join("::")
            )
        }
    }
}

impl From<&Identifier> for QualifiedName {
    fn from(id: &Identifier) -> Self {
        Self::new(vec![id.clone()], SrcRef::none())
    }
}

impl From<&str> for QualifiedName {
    fn from(value: &str) -> Self {
        Self(Refer::none(
            value.split("::").map(Identifier::from).collect(),
        ))
    }
}

impl From<QualifiedName> for String {
    fn from(value: QualifiedName) -> Self {
        value
            .iter()
            .map(|id| format!("{id}"))
            .collect::<Vec<_>>()
            .join("::")
    }
}
