// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of arguments syntax entities.

use crate::{ord_map::*, syntax::*};
use derive_more::{Deref, DerefMut};
use microcad_lang_base::{Refer, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

/// *Ordered map* of arguments in a [`Call`].
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, SrcReferrer)]
pub struct ArgumentList(pub Refer<OrdMap<Identifier, Argument>>);

impl std::fmt::Display for ArgumentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let mut v = self
                .0
                .value
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>();
            v.sort();
            v.join(", ")
        })
    }
}

impl TreeDisplay for ArgumentList {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}ArgumentList:", "")?;
        depth.indent();
        self.0.value.iter().try_for_each(|p| p.tree_print(f, depth))
    }
}

impl std::ops::Index<&Identifier> for ArgumentList {
    type Output = Argument;

    fn index(&self, name: &Identifier) -> &Self::Output {
        self.0.get(name).expect("key not found")
    }
}

impl std::ops::Index<usize> for ArgumentList {
    type Output = Argument;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0.value[idx]
    }
}
