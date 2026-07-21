// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::ir;

use microcad_lang_base::{Identifier, IsDefault, Refer, SrcRef, SrcReferrer, is_default};
use microcad_lang_proc_macros::Identifiable;

pub use microcad_lang_base::element::WorkbenchKind;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct WorkbenchStatement<NAME: Serialize = ir::SymbolPath> {
    pub attr: ir::OuterAttributes,
    #[serde(skip_serializing_if = "is_default", default)]
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    #[serde(skip_serializing_if = "is_default", default)]
    pub keyword_src_ref: SrcRef,
    pub id: Option<ir::Identifier>,
    pub ty: Option<ir::TypeAnnotation>,
    pub expression: WorkbenchExpression<NAME>,
}

#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct Group {
    pub src_ref: SrcRef,
    pub attr: ir::InnerAttributes,
    pub statements: Box<[WorkbenchStatement]>,
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Init {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Outer attributes.
    pub attr: ir::OuterAttributes,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    #[serde(skip_serializing_if = "is_default", default)]
    pub statements: Box<[WorkbenchStatement]>,
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
    Group(ir::Group),
    If(ir::If<WorkbenchExpression<NAME>, ir::Group>),
    Call(ir::Call<WorkbenchExpression<NAME>>),
    Marker(Marker),
    BinaryOp(ir::BinaryOp<WorkbenchExpression<NAME>>),
    UnaryOp(ir::UnaryOp<WorkbenchExpression<NAME>>),
    MetaAccess(Access<Identifier, NAME>),
    ArrayAccess(Access<Box<ir::ConstantExpression<NAME>>, NAME>),
    PropertyAccess(Access<Identifier, NAME>),
    MethodCall(MethodCall<NAME>),
}

impl<NAME: Serialize> ir::ExpressionKind for WorkbenchExpression<NAME> {
    type Name = NAME;
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
