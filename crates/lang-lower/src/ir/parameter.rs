// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

use crate::ir::{self, ExprSpec};

use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_types::{Tuple, Ty, Type};
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
    /// Type of the parameter
    pub ty: ir::Type,
    /// default value of the parameter or `None`
    pub default_value: Option<ir::ConstantExpression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Parameter {
    pub fn new(id: impl Into<Identifier>, ty: impl Into<ir::Type>) -> Self {
        Self {
            attr: ir::Attributes::default(),
            id: id.into(),
            ty: ty.into(),
            default_value: None,
            src_ref: SrcRef::none(),
        }
    }
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

impl ParameterList {
    /// Positional lookup by index
    pub fn get_by_index(&self, index: usize) -> Option<&Parameter> {
        self.parameters.get(index)
    }

    /// Named lookup by Identifier
    pub fn get_by_name(&self, id: &Identifier) -> Option<&Parameter> {
        self.parameters.iter().find(|p| &p.id == id)
    }

    /// Total parameter count
    pub fn len(&self) -> usize {
        self.parameters.len()
    }

    /// Check if the `ParameterList` is empty.
    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }

    /// Return default values for this parameters, assuming all constant expression have been folded into values.
    pub fn default_values(&self) -> Tuple {
        Tuple::from_iter(self.parameters.iter().filter_map(|param| {
            match param.default_value.as_ref().and_then(|expr| expr.value()) {
                Some(value) => Some((param.id.clone(), value.clone())),
                None => None,
            }
        }))
    }
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

impl<P> FromIterator<P> for ParameterList
where
    P: Into<Parameter>,
{
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Self {
            parameters: iter
                .into_iter()
                .map(|p| p.into())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            src_ref: SrcRef::none(),
        }
    }
}

impl From<Vec<ir::Parameter>> for ParameterList {
    fn from(mut params: Vec<ir::Parameter>) -> Self {
        params.sort_by(|a, b| a.id.cmp(&b.id));

        Self {
            parameters: params.into_boxed_slice(),
            src_ref: SrcRef::none(),
        }
    }
}
