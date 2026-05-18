// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::{Deref, DerefMut};

use microcad_lang_base::{Identifier, Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::SrcReferrer;
use miette::SourceSpan;

/// A *qualified name* consists of a list of *identifiers*, separated by `::`,
/// e.g. `a::b::c`
#[derive(
    Default, Clone, Debug, PartialEq, Hash, Eq, Ord, PartialOrd, DerefMut, Deref, SrcReferrer,
)]
pub struct QualifiedName(Refer<Vec<Identifier>>);

/// List of *qualified names* which can be displayed.
#[derive(Debug, Deref)]
pub struct QualifiedNames(Vec<QualifiedName>);

impl std::fmt::Display for QualifiedNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl FromIterator<QualifiedName> for QualifiedNames {
    fn from_iter<T: IntoIterator<Item = QualifiedName>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl QualifiedName {
    /// Create [`QualifiedName`] from [`Identifier`]s.
    ///
    /// - `ids`: *Identifiers* that concatenate to the *qualified name*.
    /// - `src_ref`: Reference for the whole name.
    pub fn new(ids: Vec<Identifier>, src_ref: SrcRef) -> Self {
        Self(Refer::new(ids, src_ref))
    }

    /// Returns true if self is a qualified name with multiple ids in it
    pub fn is_qualified(&self) -> bool {
        self.0.len() > 1
    }

    /// Tells if self is in a specified module
    pub fn is_within(&self, module: &QualifiedName) -> bool {
        self.starts_with(module)
    }

    /// remove the first name from path
    pub fn remove_first(&self) -> Self {
        Self(Refer::new(self.0[1..].to_vec(), self.0.src_ref.clone()))
    }

    /// remove the first name from path
    pub fn remove_last(self) -> Self {
        Self(Refer::new(
            self.0[..self.0.len() - 1].to_vec(),
            self.0.src_ref.clone(),
        ))
    }

    /// Append identifier to name
    pub fn push(&mut self, id: Identifier) {
        self.0.push(id)
    }

    /// Split name into first id and the rest
    pub fn split_first(&self) -> (Identifier, QualifiedName) {
        match self.len() {
            0 => todo!("return None or error?"),
            1 => (self.0[0].clone(), Self::default()),
            _ => (self.0[0].clone(), Self(Refer::none(self.0[1..].into()))),
        }
    }

    /// Add given prefix to name
    pub fn with_prefix(&self, prefix: &QualifiedName) -> Self {
        let mut full_name = prefix.clone();
        full_name.append(&mut self.clone());
        full_name
    }
}

impl crate::lower::SingleIdentifier for QualifiedName {
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
        Self(Refer::new(vec![id], src_ref))
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

impl From<Refer<Vec<Identifier>>> for QualifiedName {
    fn from(value: Refer<Vec<Identifier>>) -> Self {
        Self(value)
    }
}

impl FromIterator<Identifier> for QualifiedName {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        Self(Refer::none(iter.into_iter().collect()))
    }
}

impl From<&Identifier> for QualifiedName {
    fn from(id: &Identifier) -> Self {
        Self(Refer::none(vec![id.clone()]))
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
