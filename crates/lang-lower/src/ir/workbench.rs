// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{CastInto, ir};

use derive_more::{Display, From};
use microcad_lang_base::{IsDefault, Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

pub use microcad_lang_base::element::WorkbenchKind;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct WorkbenchStatement<NAME: ir::NameKind = ir::SymbolPath> {
    pub attr: ir::OuterAttributes<NAME>,
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    pub keyword_src_ref: SrcRef,
    pub id: Option<ir::Identifier>,
    pub ty: Option<ir::TypeAnnotation>,
    pub expression: WorkbenchExpression<NAME>,
}

impl<NameA: ir::NameKind, NameB: ir::NameKind> CastInto<WorkbenchStatement<NameA>>
    for WorkbenchStatement<NameB>
where
    NameB: Into<NameA>,
{
    fn cast_into(self) -> WorkbenchStatement<NameA> {
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
pub struct Group<NAME: ir::NameKind = ir::SymbolPath> {
    pub src_ref: SrcRef,
    pub attr: ir::InnerAttributes<NAME>,
    pub statements: Box<[WorkbenchStatement<NAME>]>,
}

impl<Src: ir::NameKind, Dst: ir::NameKind> CastInto<Group<Dst>> for Group<Src>
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
pub struct Init<NAME: ir::NameKind = ir::SymbolPath> {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Outer attributes.
    pub attr: ir::OuterAttributes<NAME>,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    pub statements: Box<[WorkbenchStatement<NAME>]>,
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
pub enum WorkbenchExpression<Name: ir::NameKind = ir::SymbolPath> {
    Invalid,
    Literal(ir::Literal),
    Name(Name),
    Tuple(ir::TupleExpression<WorkbenchExpression<Name>>),
    Group(ir::Group<Name>),
    If(ir::If<WorkbenchExpression<Name>>),
    Call(ir::Call<WorkbenchExpression<Name>>),
    Marker(Marker),
}

impl<Name: ir::NameKind> SrcReferrer for WorkbenchExpression<Name> {
    fn src_ref(&self) -> SrcRef {
        match &self {
            WorkbenchExpression::Invalid => SrcRef::none(),
            WorkbenchExpression::Literal(literal) => literal.src_ref(),
            WorkbenchExpression::Name(name) => name.src_ref(),
            WorkbenchExpression::Tuple(tuple_expression) => tuple_expression.src_ref,
            WorkbenchExpression::Group(group) => group.src_ref,
            WorkbenchExpression::If(if_) => if_.src_ref,
            WorkbenchExpression::Call(call) => call.src_ref,
            WorkbenchExpression::Marker(marker) => marker.src_ref,
        }
    }
}

impl<NAME: ir::NameKind> ir::ExpressionKind for WorkbenchExpression<NAME> {
    type Name = NAME;
    type Body = Group<NAME>;
}

impl<Name: ir::NameKind> CastInto<ir::WorkbenchExpression<Name>> for ir::ConstantExpression<Name> {
    fn cast_into(self: ir::ConstantExpression<Name>) -> ir::WorkbenchExpression<Name> {
        match self {
            ir::ConstantExpression::Invalid => ir::WorkbenchExpression::Invalid,
            ir::ConstantExpression::Literal(literal) => ir::WorkbenchExpression::Literal(literal),
            ir::ConstantExpression::Name(name) => ir::WorkbenchExpression::Name(name),
            ir::ConstantExpression::Tuple(_) => todo!(),
            ir::ConstantExpression::Call(call) => ir::WorkbenchExpression::Call(call.cast_into()),
        }
    }
}

impl<NameA: ir::NameKind, NameB: ir::NameKind> CastInto<WorkbenchExpression<NameA>>
    for WorkbenchExpression<NameB>
where
    NameB: Into<NameA>,
{
    fn cast_into(self) -> WorkbenchExpression<NameA> {
        use WorkbenchExpression::*;
        match self {
            Invalid => Invalid,
            Literal(literal) => Literal(literal),
            Name(name) => Name(name.into()),
            Tuple(tuple_expression) => Tuple(tuple_expression.cast_into()),
            Group(group) => Group(group.cast_into()),
            If(if_) => If(if_.cast_into()),
            Call(call) => Call(call.cast_into()),
            Marker(marker) => Marker(marker),
        }
    }
}

/// Workbench items that will be resolved into Symbols
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct WorkbenchItems {
    /// `use`
    pub aliases: ir::Aliases,
    /// `const`
    pub constants: Box<[ir::Constant]>,
    /// `fn`
    pub functions: Box<[ir::Function]>,
}

impl IsDefault for WorkbenchItems {
    fn is_default(&self) -> bool {
        self.aliases.is_default() && self.constants.is_default()
    }
}

/// Workbench definition, e.g `sketch`, `part` or `op`.
#[derive(Debug, Clone, Identifiable, Hash, PartialEq, Serialize, Deserialize)]
pub struct Workbench {
    /// SrcRef of the `sketch`/`part`/`op` keyword
    pub keyword_ref: SrcRef,
    /// Workbench outer attributes.
    pub outer_attr: ir::OuterAttributes,
    /// Visibility from outside modules.
    pub visibility: ir::Visibility,
    /// Workbench kind.
    pub kind: Refer<WorkbenchKind>,
    /// Workbench name.
    pub id: ir::Identifier,
    /// Workbench's building plan.
    pub parameters: ir::ParameterList,
    /// Workbench inner attributes
    pub inner_attr: ir::InnerAttributes,
    /// `init`
    pub inits: Box<[Init]>,
    /// Items that will be resolved into Symbols
    pub items: ir::WorkbenchItems,
    /// The actual statements to build the Model
    pub statements: Box<[ir::WorkbenchStatement]>,
}

impl SrcReferrer for Workbench {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl std::fmt::Display for Workbench {
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
