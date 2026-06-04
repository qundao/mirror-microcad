// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter list syntax element

use crate::ir;

use derive_more::{Deref, DerefMut};
use microcad_lang_base::{Identifier, OrdMap, Refer};
use microcad_lang_proc_macros::SrcReferrer;

/// Parameter list
#[derive(Clone, Debug, Default, Deref, DerefMut, SrcReferrer)]
pub struct ParameterList(pub Refer<OrdMap<Identifier, ir::Parameter>>);

impl ParameterList {
    /// Return ids of all parameters
    pub fn ids(&self) -> impl Iterator<Item = Identifier> {
        self.keys().cloned()
    }

    /// Return if given identifier is in parameter list
    pub fn contains_key(&self, id: &Identifier) -> bool {
        self.iter().any(|p| *id == p.id)
    }
}

impl std::fmt::Display for ParameterList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
