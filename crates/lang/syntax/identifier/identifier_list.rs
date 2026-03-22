// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::syntax::*;
use derive_more::{Deref, DerefMut};
use microcad_lang_base::{Refer, SrcRef};
use microcad_lang_proc_macros::SrcReferrer;

/// A list of identifiers
///
/// Used e.g. for multiple variable declarations.
/// Cannot contain duplicates.
#[derive(Default, Debug, Clone, PartialEq, Deref, DerefMut, SrcReferrer)]
pub struct IdentifierList(pub Refer<Vec<Identifier>>);

impl IdentifierList {
    /// Create new identifier list
    pub fn new(identifiers: Vec<Identifier>, src_ref: SrcRef) -> Self {
        Self(Refer::new(identifiers, src_ref))
    }
}

impl FromIterator<Identifier> for IdentifierList {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        let v: Vec<_> = iter.into_iter().collect();
        let src_ref = SrcRef::merge_all(v.iter());
        Self(Refer::new(v, src_ref))
    }
}

impl std::fmt::Display for IdentifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut sorted = self.0.clone();
        sorted.sort();
        write!(
            f,
            "{}",
            sorted
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl std::iter::IntoIterator for IdentifierList {
    type Item = Identifier;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
