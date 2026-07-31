// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value list evaluation entity

use derive_more::Deref;
use microcad_lang_base::{HashMap, Identifier};

use crate::ParameterValue;

/// List of parameter values
#[derive(Clone, Debug, Default, Deref)]
pub struct ParameterValueList(HashMap<Identifier, ParameterValue>);

impl<I, P> FromIterator<(I, P)> for ParameterValueList
where
    I: Into<Identifier>,
    P: Into<ParameterValue>,
{
    fn from_iter<T: IntoIterator<Item = (I, P)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(i, p)| (i.into(), p.into()))
                .collect(),
        )
    }
}
