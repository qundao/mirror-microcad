// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{CastInto, ir};

use microcad_lang_base::{Identifier, IsDefault, Refer, SrcRef, SrcReferrer, is_default};
use microcad_lang_proc_macros::Identifiable;

pub use microcad_lang_base::element::WorkbenchKind;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct WorkbenchStatement<NAME: Serialize = ir::SymbolPath> {
    pub attr: ir::OuterAttributes<NAME>,
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    pub keyword_src_ref: SrcRef,
    pub id: Option<ir::Identifier>,
    pub ty: Option<ir::TypeAnnotation>,
    pub expression: WorkbenchExpression<NAME>,
}

impl<NameA: Serialize, NameB: Serialize> CastInto<WorkbenchStatement<NameA>>
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
pub struct Group<NAME: Serialize = ir::SymbolPath> {
    pub src_ref: SrcRef,
    pub attr: ir::InnerAttributes<NAME>,
    pub statements: Box<[WorkbenchStatement<NAME>]>,
}

impl<Src: Serialize, Dst: Serialize> CastInto<Group<Dst>> for Group<Src>
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
pub struct Init<NAME: Serialize = ir::SymbolPath> {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Outer attributes.
    pub attr: ir::OuterAttributes<NAME>,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    #[serde(skip_serializing_if = "is_default", default)]
    pub statements: Box<[WorkbenchStatement<NAME>]>,
    /// Source reference
    pub src_ref: SrcRef,
}

/// Node marker, e.g. `@input`.
#[derive(Debug, Clone, PartialEq, Serialize, Hash, Deserialize)]
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

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.id)
    }
}

type Access<ELEMENT, NAME> = ir::ElementAccess<WorkbenchExpression<NAME>, ELEMENT>;
type MethodCall<NAME> = Access<ir::Call<WorkbenchExpression<NAME>>, NAME>;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum WorkbenchExpression<NAME: Serialize = ir::SymbolPath> {
    Invalid,
    Literal(ir::Literal),
    Name(NAME),
    FormatString(ir::FormatString<NAME>),
    ArrayExpression(ir::ArrayExpression<WorkbenchExpression<NAME>>),
    TupleExpression(ir::TupleExpression<WorkbenchExpression<NAME>>),
    Group(ir::Group<NAME>),
    If(ir::If<WorkbenchExpression<NAME>>),
    Call(ir::Call<WorkbenchExpression<NAME>>),
    Marker(Marker),
    BinaryOp(ir::BinaryOp<WorkbenchExpression<NAME>>),
    UnaryOp(ir::UnaryOp<WorkbenchExpression<NAME>>),
    MetaAccess(Access<Identifier, NAME>),
    ArrayAccess(Access<ir::ConstantExpression<NAME>, NAME>),
    PropertyAccess(Access<Identifier, NAME>),
    MethodCall(MethodCall<NAME>),
}

impl<NAME: Serialize> ir::ExpressionKind for WorkbenchExpression<NAME> {
    type Name = NAME;
    type Body = Group<NAME>;
}

impl<NameA: Serialize, NameB: Serialize> CastInto<WorkbenchExpression<NameA>>
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
            FormatString(format_string) => FormatString(format_string.cast_into()),
            ArrayExpression(array_expression) => ArrayExpression(array_expression.cast_into()),
            TupleExpression(tuple_expression) => TupleExpression(tuple_expression.cast_into()),
            Group(group) => Group(group.cast_into()),
            If(if_) => If(if_.cast_into()),
            Call(call) => Call(call.cast_into()),
            Marker(marker) => Marker(marker),
            BinaryOp(binary_op) => BinaryOp(binary_op.cast_into()),
            UnaryOp(unary_op) => UnaryOp(unary_op.cast_into()),
            MetaAccess(element_access) => MetaAccess(element_access.cast_into()),
            ArrayAccess(element_access) => ArrayAccess(element_access.cast_into()),
            PropertyAccess(element_access) => PropertyAccess(element_access.cast_into()),
            MethodCall(element_access) => MethodCall(element_access.cast_into()),
        }
    }
}

/// Workbench items that will be resolved into Symbols
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct WorkbenchItems {
    /// `use`
    #[serde(skip_serializing_if = "is_default", default)]
    pub aliases: ir::Aliases,
    /// `const`
    #[serde(skip_serializing_if = "is_default", default)]
    pub constants: Box<[ir::Constant]>,
    /// `fn`
    #[serde(skip_serializing_if = "is_default", default)]
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
    #[serde(skip_serializing_if = "is_default", default)]
    pub inits: Box<[Init]>,
    /// Items that will be resolved into Symbols
    #[serde(skip_serializing_if = "is_default", default)]
    pub items: ir::WorkbenchItems,
    /// The actual statements to build the Model
    #[serde(skip_serializing_if = "is_default", default)]
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
