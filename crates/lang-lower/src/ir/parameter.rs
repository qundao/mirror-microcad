// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

use crate::ir;

use microcad_lang_base::{Identifier, OrdMap, OrdMapValue, Refer, SrcRef};
use microcad_lang_proc_macros::{Identifiable, SrcReferrer};

use derive_more::{Deref, DerefMut};

/// A parameter of a parameter list.
#[derive(Debug, Default, SrcReferrer, Identifiable)]
pub struct Parameter {
    /// Parameter attributes
    pub attr: ir::Attributes,
    /// Name of the parameter
    pub(crate) id: Identifier,
    /// Type of the parameter or `None`
    pub specified_type: Option<ir::TypeAnnotation>,
    /// default value of the parameter or `None`
    pub default_value: Option<ir::ConstantExpression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl OrdMapValue<Identifier> for Parameter {
    fn key(&self) -> Option<Identifier> {
        Some(self.id.clone())
    }
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

/// Parameter list
#[derive(Debug, Default, Deref, DerefMut, SrcReferrer)]
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
