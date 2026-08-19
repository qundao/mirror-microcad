// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value evaluation entity

use derive_more::Deref;
use microcad_lang_base::{Identifier, SrcRef};
use microcad_macros::SrcReferrer;
use microcad_lang_types::{Ty, Type, Value};
use serde::{Deserialize, Serialize};

/// Parameter value is the result of evaluating a parameter
#[derive(Clone, Debug, Hash, PartialEq, SrcReferrer, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter id
    pub id: Identifier,
    /// Documentation
    pub doc: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Hash, Default, Deref, Serialize, Deserialize)]
pub struct ParameterList(Box<[Parameter]>);

impl ParameterList {
    /// Positional lookup by index
    pub fn get_by_index(&self, index: usize) -> Option<&Parameter> {
        self.0.get(index)
    }

    /// Named lookup by Identifier
    pub fn get_by_name(&self, id: &Identifier) -> Option<&Parameter> {
        self.0.iter().find(|p| &p.id == id)
    }

    /// Total parameter count
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<P> FromIterator<P> for ParameterList
where
    P: Into<Parameter>,
{
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|p| p.into())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }
}
