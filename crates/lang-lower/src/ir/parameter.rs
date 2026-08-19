// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

use crate::ir;

use microcad_lang_base::{Identifier, SrcRef};
use microcad_macros::{Identifiable, SrcReferrer};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A parameter of a parameter list.
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, SrcReferrer, Identifiable, PartialEq, Serialize, Deserialize)]

pub struct Parameter {
    /// Parameter attributes
    pub attr: ir::Attributes,
    /// Name of the parameter
    pub id: Identifier,
    /// Type of the parameter or `None`
    pub ty: ir::Type,
    /// default value of the parameter or `None`
    pub default_value: Option<ir::ConstantExpression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{id}: {ty}", id = self.id, ty = self.ty)?;
        match &self.default_value {
            Some(v) => write!(f, " = {v}"),
            _ => Ok(()),
        }
    }
}

/// Parameter list, sorted by id.
#[derive(Debug, Clone, SrcReferrer, Hash, PartialEq, Serialize, Deserialize)]
pub struct ParameterList {
    pub parameters: Box<[ir::Parameter]>,
    pub src_ref: SrcRef,
}

impl std::fmt::Display for ParameterList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
