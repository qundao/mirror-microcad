// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value evaluation entity

use derive_more::Deref;
use microcad_lang_base::{HashMap, Identifier, SrcRef};
use microcad_lang_proc_macros::SrcReferrer;
use microcad_lang_types::{Ty, Type, Value};
use serde::{Deserialize, Serialize};

use crate::rst::DocBlock;

/// Parameter value is the result of evaluating a parameter
#[derive(Clone, Debug, Hash, PartialEq, SrcReferrer, Serialize, Deserialize)]
pub struct Parameter {
    pub doc: Option<DocBlock>,
    /// Parameter type
    pub ty: Option<Type>,
    /// Parameter default
    pub default_value: Option<Value>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Ty for Parameter {
    /// Return effective type
    ///
    /// Returns any `specified_type` or the type of the `default_value`.
    /// Panics if neither of both is available.
    fn ty(&self) -> Type {
        if let Some(ty) = &self.ty {
            ty.clone()
        } else if let Some(def) = &self.default_value {
            def.ty()
        } else {
            Type::Invalid
        }
    }
}

/// List of parameter values
#[derive(Clone, Debug, PartialEq, Default, Deref, Serialize, Deserialize)]
pub struct ParameterList(HashMap<Identifier, Parameter>);

impl<I, P> FromIterator<(I, P)> for ParameterList
where
    I: Into<Identifier>,
    P: Into<Parameter>,
{
    fn from_iter<T: IntoIterator<Item = (I, P)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(i, p)| (i.into(), p.into()))
                .collect(),
        )
    }
}

impl std::hash::Hash for ParameterList {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Collect and sort key-value pairs to guarantee a deterministic hash
        let mut entries: Vec<_> = self.0.iter().collect();
        entries.sort_by_key(|(k, _)| *k);

        entries.len().hash(state); // Hash length first
        for (k, v) in entries {
            k.hash(state);
            v.hash(state);
        }
    }
}
