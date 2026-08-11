// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{CastInto, ir};

use derive_more::{Display, From};
use microcad_lang_base::{Refer, SingleIdentifier, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

pub use microcad_lang_base::element::WorkbenchKind;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct WorkbenchStatement<Name: ir::NameSpec = ir::Name> {
    pub attr: ir::OuterAttributes<Name>,
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    pub keyword_src_ref: SrcRef,
    pub id: Option<ir::Identifier>,
    pub ty: ir::Type,
    pub expression: WorkbenchExpression<Name>,
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<WorkbenchStatement<Dst>>
    for WorkbenchStatement<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> WorkbenchStatement<Dst> {
        WorkbenchStatement {
            attr: self.attr.cast_into(),
            src_ref: self.src_ref,
            visibility: self.visibility,
            keyword_src_ref: self.keyword_src_ref,
            id: self.id,
            ty: self.ty,
            expression: self.expression.cast_into(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct Group<Name: ir::NameSpec = ir::Name> {
    pub src_ref: SrcRef,
    pub attr: ir::InnerAttributes<Name>,
    pub statements: Box<[WorkbenchStatement<Name>]>,
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<Group<Dst>> for Group<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> Group<Dst> {
        Group {
            src_ref: self.src_ref,
            attr: self.attr.cast_into(),
            statements: self.statements.cast_into(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Init<Name: ir::NameSpec = ir::Name> {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Outer attributes.
    pub attr: ir::OuterAttributes<Name>,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    pub statements: Box<[WorkbenchStatement<Name>]>,
    /// Source reference
    pub src_ref: SrcRef,
}

/// Node marker, e.g. `@input`.
#[derive(Debug, Display, Clone, PartialEq, Serialize, Hash, Deserialize)]
#[display("@{}", id)]
pub struct Marker {
    /// Marker name, e.g. `input`
    pub id: ir::Identifier,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Marker {
    /// Returns true if the marker is an input placeholder
    pub fn is_input_placeholder(&self) -> bool {
        &self.id == "input"
    }
}

#[derive(Debug, Clone, From, PartialEq, Hash, Serialize, Deserialize)]
pub enum WorkbenchExpression<Name: ir::NameSpec = ir::Name> {
    Invalid,
    Literal(ir::Literal),
    Name(Name),
    Group(ir::Group<Name>),
    If(ir::If<WorkbenchExpression<Name>>),
    Call(ir::Call<WorkbenchExpression<Name>>),
    Marker(Marker),
}

impl<Name: ir::NameSpec> SrcReferrer for WorkbenchExpression<Name> {
    fn src_ref(&self) -> SrcRef {
        use WorkbenchExpression::*;
        match &self {
            Invalid => SrcRef::none(),
            Literal(literal) => literal.src_ref(),
            Name(name) => name.src_ref(),
            Group(group) => group.src_ref,
            If(if_) => if_.src_ref,
            Call(call) => call.src_ref,
            Marker(marker) => marker.src_ref,
        }
    }
}

impl<Name: ir::NameSpec> ir::ExprSpec for WorkbenchExpression<Name> {
    type Name = Name;
    type Body = Group<Name>;
}

impl<Name: ir::NameSpec> SingleIdentifier for WorkbenchExpression<Name> {
    fn single_identifier(&self) -> Option<&microcad_lang_base::Identifier> {
        match self {
            WorkbenchExpression::Name(name) => name.single_identifier(),
            _ => None,
        }
    }
}

impl<Name: ir::NameSpec> CastInto<ir::WorkbenchExpression<Name>> for ir::ConstantExpression<Name> {
    fn cast_into(self: ir::ConstantExpression<Name>) -> ir::WorkbenchExpression<Name> {
        match self {
            ir::ConstantExpression::Invalid => ir::WorkbenchExpression::Invalid,
            ir::ConstantExpression::Literal(literal) => ir::WorkbenchExpression::Literal(literal),
            ir::ConstantExpression::Name(name) => ir::WorkbenchExpression::Name(name),
            ir::ConstantExpression::Call(call) => ir::WorkbenchExpression::Call(call.cast_into()),
        }
    }
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<WorkbenchExpression<Dst>>
    for WorkbenchExpression<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> WorkbenchExpression<Dst> {
        use WorkbenchExpression::*;
        match self {
            Invalid => Invalid,
            Literal(literal) => Literal(literal),
            Name(name) => Name(name.into()),
            Group(group) => Group(group.cast_into()),
            If(if_) => If(if_.cast_into()),
            Call(call) => Call(call.cast_into()),
            Marker(marker) => Marker(marker),
        }
    }
}

/// Workbench items that will be resolved into Symbols
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct WorkbenchItems<Name: ir::NameSpec = ir::Name> {
    /// `use`
    pub aliases: ir::Aliases<Name>,
    /// `const`
    pub constants: Box<[ir::Constant<Name>]>,
    /// `fn`
    pub functions: Box<[ir::Function<Name>]>,
}

/// Workbench definition, e.g `sketch`, `part` or `op`.
#[derive(Debug, Clone, Identifiable, Hash, PartialEq, Serialize, Deserialize)]
pub struct Workbench<Name: ir::NameSpec = ir::Name> {
    /// SrcRef of the `sketch`/`part`/`op` keyword
    pub keyword_ref: SrcRef,
    /// Workbench outer attributes.
    pub outer_attr: ir::OuterAttributes<Name>,
    /// Visibility from outside modules.
    pub visibility: ir::Visibility,
    /// Workbench kind.
    pub kind: Refer<WorkbenchKind>,
    /// Workbench name.
    pub id: ir::Identifier,
    /// Workbench's building plan.
    pub parameters: ir::ParameterList<Name>,
    /// Workbench inner attributes
    pub inner_attr: ir::InnerAttributes<Name>,
    /// `init`
    pub inits: Box<[Init<Name>]>,
    /// Items that will be resolved into Symbols
    pub items: ir::WorkbenchItems<Name>,
    /// The actual statements to build the Model
    pub statements: Box<[ir::WorkbenchStatement<Name>]>,
}

impl<Name: ir::NameSpec> SrcReferrer for Workbench<Name> {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl<Name: ir::NameSpec> std::fmt::Display for Workbench<Name>
where
    Name: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{visibility}{kind} {id}({parameters})",
            visibility = self.visibility,
            kind = self.kind,
            id = self.id,
            parameters = self.parameters,
        )
    }
}
