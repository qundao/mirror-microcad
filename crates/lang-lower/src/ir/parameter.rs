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
#[serde(bound(serialize = "Name: Serialize", deserialize = "Name: Deserialize<'de>"))]
pub struct Parameter<Name: ir::NameSpec = ir::Name> {
    /// Parameter attributes
    pub attr: ir::OuterAttributes<Name>,
    /// Name of the parameter
    pub id: Identifier,
    /// Type of the parameter or `None`
    pub ty: ir::Type,
    /// default value of the parameter or `None`
    pub default_value: Option<ir::ConstantExpression<Name>>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<Name: ir::NameSpec> std::fmt::Display for Parameter<Name>
where
    Name: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{id}: {ty}", id = self.id, ty = self.ty)?;
        match &self.default_value {
            Some(v) => write!(f, " = {v}"),
            _ => Ok(()),
        }
    }
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<Parameter<Dst>> for Parameter<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> Parameter<Dst> {
        Parameter {
            attr: self.attr.cast_into(),
            id: self.id,
            ty: self.ty,
            default_value: self.default_value.map(|v| v.cast_into()),
            src_ref: self.src_ref,
        }
    }
}

/// Parameter list, sorted by id.
#[derive(Debug, Clone, SrcReferrer, Hash, PartialEq, Serialize, Deserialize)]
pub struct ParameterList<Name: ir::NameSpec = ir::Name>(
    pub Refer<Box<[ir::Parameter<Name>]>>,
);

impl<Name: ir::NameSpec> ParameterList<Name> {
    /// Return ids of all parameters
    pub fn ids(&self) -> impl Iterator<Item = Identifier> {
        self.0.iter().map(|param| param.id.clone())
    }

    /// Return if given identifier is in parameter list
    pub fn contains_key(&self, id: &Identifier) -> bool {
        self.ids().any(|p_id| *id == p_id)
    }
}

impl<Name: ir::NameSpec> std::fmt::Display for ParameterList<Name>
where
    Name: std::fmt::Display,
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

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<ParameterList<Dst>> for ParameterList<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> ParameterList<Dst> {
        ParameterList(self.0.cast_into())
    }
}
