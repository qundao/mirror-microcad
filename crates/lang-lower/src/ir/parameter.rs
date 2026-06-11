// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

use crate::ir;

use microcad_lang_base::{Identifier, Refer, SrcRef};
use microcad_lang_proc_macros::{Identifiable, SrcReferrer};

use crate::is_default;
use derive_more::Deref;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// A parameter of a parameter list.
#[skip_serializing_none]
#[derive(Debug, Default, SrcReferrer, Identifiable, Serialize)]
pub struct Parameter {
    /// Parameter attributes
    #[serde(skip_serializing_if = "is_default", default)]
    pub attr: ir::OuterAttributes,
    /// Name of the parameter
    pub(crate) id: Identifier,
    /// Type of the parameter or `None`
    pub specified_type: Option<ir::TypeAnnotation>,
    /// default value of the parameter or `None`
    pub default_value: Option<ir::ConstantExpression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(t), Some(v)) => write!(f, "{}: {t} = {v}", self.id),
            (Some(t), None) => write!(f, "{}: {t}", self.id),
            (None, Some(v)) => write!(f, "{} = {v}", self.id),
            _ => Ok(()),
        }
    }
}

/// Parameter list, sorted by id.
#[derive(Debug, Default, Deref, SrcReferrer, Serialize)]
pub struct ParameterList(pub Refer<Box<[ir::Parameter]>>);

impl ParameterList {
    /// Return ids of all parameters
    pub fn ids(&self) -> impl Iterator<Item = Identifier> {
        self.iter().map(|param| param.id.clone())
    }

    /// Return if given identifier is in parameter list
    pub fn contains_key(&self, id: &Identifier) -> bool {
        self.ids().any(|p_id| *id == p_id)
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
