// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{IsDefault, ir, is_default};

use microcad_lang_base::{Identifier, Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

pub use microcad_lang_base::element::WorkbenchKind;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct WorkbenchStatement {
    pub attr: ir::OuterAttributes,
    #[serde(skip_serializing_if = "is_default", default)]
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    #[serde(skip_serializing_if = "is_default", default)]
    pub keyword_src_ref: SrcRef,
    pub id: Option<ir::Identifier>,
    pub ty: Option<ir::TypeAnnotation>,
    pub expression: WorkbenchExpression,
}

#[derive(Debug, Serialize)]
pub struct WorkbenchStatements(pub Box<[WorkbenchStatement]>);

impl IsDefault for WorkbenchStatements {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}

#[derive(Debug, Serialize)]
pub struct Group(pub Refer<WorkbenchStatements>);

#[derive(Debug, Serialize)]
pub struct Init {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Outer attributes.
    #[serde(skip_serializing_if = "is_default", default)]
    pub attr: ir::OuterAttributes,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    #[serde(skip_serializing_if = "is_default", default)]
    pub statements: WorkbenchStatements,
    /// Source reference
    pub src_ref: SrcRef,
}

#[derive(Debug, Serialize, Default)]
pub struct Inits(pub Box<[Init]>);

impl IsDefault for Inits {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}

/// Node marker, e.g. `@input`.
#[derive(Debug, Serialize)]
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

type Access<ELEMENT, NAME = ir::QualifiedName> =
    ir::ElementAccess<WorkbenchExpression<NAME>, ELEMENT>;
type MethodCall<NAME = ir::QualifiedName> = ir::Call<WorkbenchExpression, NAME>;

#[derive(Debug, Serialize)]
pub enum WorkbenchExpression<NAME = ir::QualifiedName> {
    Invalid,
    Literal(ir::Literal),
    Name(NAME),
    FormatString(ir::FormatString),
    ArrayExpression(ir::ArrayExpression<WorkbenchExpression<NAME>>),
    TupleExpression(ir::TupleExpression<WorkbenchExpression<NAME>>),
    Group(ir::Group),
    If(ir::If<WorkbenchExpression<NAME>, ir::Group>),
    Call(ir::Call<WorkbenchExpression<NAME>>),
    Marker(Marker),
    BinaryOp(ir::BinaryOp<WorkbenchExpression<NAME>>),
    UnaryOp(ir::UnaryOp<WorkbenchExpression<NAME>>),
    MetaAccess(Access<Identifier>),
    ArrayAccess(Access<Box<ir::ConstantExpression<NAME>>>),
    PropertyAccess(Access<Identifier>),
    MethodCall(Access<MethodCall>),
}

/// Workbench definition, e.g `sketch`, `part` or `op`.
#[derive(Debug, Identifiable, Serialize)]
pub struct Workbench {
    /// SrcRef of the `sketch`/`part`/`op` keyword
    pub keyword_ref: SrcRef,
    /// Workbench outer attributes.
    #[serde(skip_serializing_if = "is_default", default)]
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
    #[serde(skip_serializing_if = "is_default", default)]
    pub inner_attr: ir::InnerAttributes,
    /// `use`
    #[serde(skip_serializing_if = "is_default", default)]
    pub aliases: ir::Aliases,
    /// `const`
    #[serde(skip_serializing_if = "is_default", default)]
    pub constants: ir::Constants,
    /// `init`
    #[serde(skip_serializing_if = "is_default", default)]
    pub inits: ir::Inits,
    /// The actual statements building the model
    #[serde(skip_serializing_if = "is_default", default)]
    pub statements: ir::WorkbenchStatements,
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

#[derive(Debug, Default, Serialize)]
pub struct Workbenches(pub Box<[Workbench]>);

impl IsDefault for Workbenches {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}
