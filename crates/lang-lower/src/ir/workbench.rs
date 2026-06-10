// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::ir::{self, ConstantExpression};

use microcad_lang_base::{Identifier, Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

pub use microcad_lang_base::element::WorkbenchKind;

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[derive(Debug)]
pub struct WorkbenchStatement {
    pub attr: ir::Attributes,
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    pub keyword_src_ref: SrcRef,
    pub id: Option<ir::Identifier>,
    pub ty: Option<ir::TypeAnnotation>,
    pub expression: WorkbenchExpression,
}

#[derive(Debug)]
pub struct WorkbenchStatements(pub Box<[WorkbenchStatement]>);

#[derive(Debug)]
pub struct Group(pub Refer<Box<[WorkbenchStatement]>>);

#[derive(Debug)]
pub struct InitBody(pub Vec<ir::LocalAssignment<ir::WorkbenchExpression>>);
#[derive(Debug)]
pub struct Init {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Outer attributes.
    pub attr: ir::Attributes,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    pub body: InitBody,
    /// Source reference
    pub src_ref: SrcRef,
}

#[derive(Debug)]
pub struct Inits(pub Vec<Init>);

/// Node marker, e.g. `@input`.
#[derive(Debug)]
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

type Access<ELEMENT> = ir::ElementAccess<WorkbenchExpression, ELEMENT>;
type MethodCall = ir::Call<WorkbenchExpression>;

#[derive(Debug)]
pub enum WorkbenchExpression {
    Invalid,
    Literal(ir::Literal),
    QualifiedName(ir::QualifiedName),
    FormatString(ir::FormatString),
    ArrayExpression(ir::ArrayExpression<WorkbenchExpression>),
    TupleExpression(ir::TupleExpression<WorkbenchExpression>),
    Group(ir::Group),
    If(ir::If<WorkbenchExpression, ir::Group>),
    Call(ir::Call<WorkbenchExpression>),
    Marker(Marker),
    BinaryOp(ir::BinaryOp<WorkbenchExpression>),
    UnaryOp(ir::UnaryOp<WorkbenchExpression>),
    MetaAccess(Access<Identifier>),
    ArrayAccess(Access<Box<ConstantExpression>>),
    PropertyAccess(Access<Identifier>),
    MethodCall(Access<MethodCall>),
}

/// Workbench definition, e.g `sketch`, `part` or `op`.
#[derive(Debug, Identifiable)]
pub struct Workbench {
    /// SrcRef of the `sketch`/`part`/`op` keyword
    pub keyword_ref: SrcRef,
    /// Workbench outer attributes.
    pub outer_attr: ir::Attributes,

    /// Visibility from outside modules.
    pub visibility: ir::Visibility,
    /// Workbench kind.
    pub kind: Refer<WorkbenchKind>,
    /// Workbench name.
    pub(crate) id: ir::Identifier,
    /// Workbench's building plan.
    pub parameters: ir::ParameterList,

    /// Workbench inner attributes
    pub inner_attr: ir::Attributes,

    pub aliases: ir::Aliases,

    pub constants: ir::Constants,

    pub inits: ir::Inits,

    pub statements: Vec<WorkbenchStatement>,
}

impl Workbench {
    pub(crate) fn possible_params(&self) -> Vec<String> {
        std::iter::once(&self.parameters)
            .chain(self.inits.0.iter().map(|init| &init.parameters))
            .map(|params| format!("{}( {})", self.id, params))
            .collect()
    }
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

#[derive(Debug, Default)]
pub struct Workbenches(pub Vec<Workbench>);
