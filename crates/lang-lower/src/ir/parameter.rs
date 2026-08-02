// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

use crate::{CastInto, ir};

use microcad_lang_base::{Identifier, Refer, SrcRef};
use microcad_lang_proc_macros::{Identifiable, SrcReferrer};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A parameter of a parameter list.
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, SrcReferrer, Identifiable, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "NAME: Serialize", deserialize = "NAME: Deserialize<'de>"))]
pub struct Parameter<NAME: ir::NameKind = ir::SymbolPath> {
    /// Parameter attributes
    pub attr: ir::OuterAttributes<NAME>,
    /// Name of the parameter
    pub id: Identifier,
    /// Type of the parameter or `None`
    pub specified_type: Option<ir::TypeAnnotation>,
    /// default value of the parameter or `None`
    pub default_value: Option<ir::ConstantExpression<NAME>>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<NAME: ir::NameKind> std::fmt::Display for Parameter<NAME>
where
    NAME: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(t), Some(v)) => write!(f, "{}: {t} = {v}", self.id),
            (Some(t), None) => write!(f, "{}: {t}", self.id),
            (None, Some(v)) => write!(f, "{} = {v}", self.id),
            _ => Ok(()),
        }
    }
}

impl<T: ir::NameKind, NAME: ir::NameKind> CastInto<Parameter<T>> for Parameter<NAME>
where
    NAME: Into<T>,
{
    fn cast_into(self) -> Parameter<T> {
        Parameter {
            attr: self.attr.cast_into(),
            id: self.id,
            specified_type: self.specified_type,
            default_value: self.default_value.map(|v| v.cast_into()),
            src_ref: self.src_ref,
        }
    }
}

/// Parameter list, sorted by id.
#[derive(Debug, Clone, SrcReferrer, Hash, PartialEq, Serialize, Deserialize)]
pub struct ParameterList<NAME: ir::NameKind = ir::SymbolPath>(
    pub Refer<Box<[ir::Parameter<NAME>]>>,
);

impl<NAME: ir::NameKind> ParameterList<NAME> {
    /// Return ids of all parameters
    pub fn ids(&self) -> impl Iterator<Item = Identifier> {
        self.0.iter().map(|param| param.id.clone())
    }

    /// Return if given identifier is in parameter list
    pub fn contains_key(&self, id: &Identifier) -> bool {
        self.ids().any(|p_id| *id == p_id)
    }
}

impl<NAME: ir::NameKind> std::fmt::Display for ParameterList<NAME>
where
    NAME: std::fmt::Display,
{
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

impl<T: ir::NameKind, NAME: ir::NameKind> CastInto<ParameterList<T>> for ParameterList<NAME>
where
    NAME: Into<T>,
{
    fn cast_into(self) -> ParameterList<T> {
        ParameterList(self.0.cast_into())
    }
}
