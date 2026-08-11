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
pub struct WorkbenchStatement<Path: ir::PathSpec = ir::Path> {
    pub attr: ir::OuterAttributes<Path>,
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    pub keyword_src_ref: SrcRef,
    pub id: Option<ir::Identifier>,
    pub ty: ir::Type,
    pub expression: WorkbenchExpression<Path>,
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<WorkbenchStatement<Dst>>
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
pub struct Group<Path: ir::PathSpec = ir::Path> {
    pub src_ref: SrcRef,
    pub attr: ir::InnerAttributes<Path>,
    pub statements: Box<[WorkbenchStatement<Path>]>,
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<Group<Dst>> for Group<Src>
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
pub struct Init<Path: ir::PathSpec = ir::Path> {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Outer attributes.
    pub attr: ir::OuterAttributes<Path>,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    pub statements: Box<[WorkbenchStatement<Path>]>,
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
pub enum WorkbenchExpression<Path: ir::PathSpec = ir::Path> {
    Invalid,
    Literal(ir::Literal),
    Path(Path),
    Group(ir::Group<Path>),
    If(ir::If<WorkbenchExpression<Path>>),
    Call(ir::Call<WorkbenchExpression<Path>>),
    Marker(Marker),
}

impl<Path: ir::PathSpec> SrcReferrer for WorkbenchExpression<Path> {
    fn src_ref(&self) -> SrcRef {
        use WorkbenchExpression::*;
        match &self {
            Invalid => SrcRef::none(),
            Literal(literal) => literal.src_ref(),
            Path(name) => name.src_ref(),
            Group(group) => group.src_ref,
            If(if_) => if_.src_ref,
            Call(call) => call.src_ref,
            Marker(marker) => marker.src_ref,
        }
    }
}

impl<Path: ir::PathSpec> ir::ExprSpec for WorkbenchExpression<Path> {
    type Path = Path;
    type Body = Group<Path>;
}

impl<Path: ir::PathSpec> SingleIdentifier for WorkbenchExpression<Path> {
    fn single_identifier(&self) -> Option<&microcad_lang_base::Identifier> {
        match self {
            WorkbenchExpression::Path(name) => name.single_identifier(),
            _ => None,
        }
    }
}

impl<Path: ir::PathSpec> CastInto<ir::WorkbenchExpression<Path>> for ir::ConstantExpression<Path> {
    fn cast_into(self: ir::ConstantExpression<Path>) -> ir::WorkbenchExpression<Path> {
        match self {
            ir::ConstantExpression::Invalid => ir::WorkbenchExpression::Invalid,
            ir::ConstantExpression::Literal(literal) => ir::WorkbenchExpression::Literal(literal),
            ir::ConstantExpression::Path(name) => ir::WorkbenchExpression::Path(name),
            ir::ConstantExpression::Call(call) => ir::WorkbenchExpression::Call(call.cast_into()),
        }
    }
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<WorkbenchExpression<Dst>>
    for WorkbenchExpression<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> WorkbenchExpression<Dst> {
        use WorkbenchExpression::*;
        match self {
            Invalid => Invalid,
            Literal(literal) => Literal(literal),
            Path(name) => Path(name.into()),
            Group(group) => Group(group.cast_into()),
            If(if_) => If(if_.cast_into()),
            Call(call) => Call(call.cast_into()),
            Marker(marker) => Marker(marker),
        }
    }
}

/// Workbench items that will be resolved into Symbols
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct WorkbenchItems<Path: ir::PathSpec = ir::Path> {
    /// `use`
    pub aliases: ir::Aliases<Path>,
    /// `const`
    pub constants: Box<[ir::Constant<Path>]>,
    /// `fn`
    pub functions: Box<[ir::Function<Path>]>,
}

/// Workbench definition, e.g `sketch`, `part` or `op`.
#[derive(Debug, Clone, Identifiable, Hash, PartialEq, Serialize, Deserialize)]
pub struct Workbench<Path: ir::PathSpec = ir::Path> {
    /// SrcRef of the `sketch`/`part`/`op` keyword
    pub keyword_ref: SrcRef,
    /// Workbench outer attributes.
    pub outer_attr: ir::OuterAttributes<Path>,
    /// Visibility from outside modules.
    pub visibility: ir::Visibility,
    /// Workbench kind.
    pub kind: Refer<WorkbenchKind>,
    /// Workbench name.
    pub id: ir::Identifier,
    /// Workbench's building plan.
    pub parameters: ir::ParameterList<Path>,
    /// Workbench inner attributes
    pub inner_attr: ir::InnerAttributes<Path>,
    /// `init`
    pub inits: Box<[Init<Path>]>,
    /// Items that will be resolved into Symbols
    pub items: ir::WorkbenchItems<Path>,
    /// The actual statements to build the Model
    pub statements: Box<[ir::WorkbenchStatement<Path>]>,
}

impl<Path: ir::PathSpec> SrcReferrer for Workbench<Path> {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl<Path: ir::PathSpec> std::fmt::Display for Workbench<Path>
where
    Path: std::fmt::Display,
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
