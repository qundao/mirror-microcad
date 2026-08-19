// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{CastInto, ir};

use derive_more::{Display, From};
use microcad_lang_base::{SingleIdentifier, SrcRef, SrcReferrer};

pub use microcad_lang_base::element::WorkbenchKind;
use microcad_lang_types::Value;
use serde::{Deserialize, Serialize};

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct WorkbenchStatement {
    pub attr: ir::Attributes,
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    pub keyword_src_ref: SrcRef,
    pub name: Option<ir::Identifier>,
    pub ty: ir::Type,
    pub expression: WorkbenchExpression,
}

/// Builder methods for testing
impl WorkbenchStatement {
    pub fn new(expr: impl Into<WorkbenchExpression>) -> Self {
        Self {
            attr: Default::default(),
            src_ref: Default::default(),
            visibility: Default::default(),
            keyword_src_ref: Default::default(),
            name: None,
            ty: Default::default(),
            expression: expr.into(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct Group {
    pub src_ref: SrcRef,
    pub attr: ir::Attributes,
    pub statements: Box<[WorkbenchStatement]>,
}

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct InitStatement {
    /// Property name
    pub name: ir::Identifier,
    /// Property value
    pub expression: WorkbenchExpression,
    /// Source code reference
    pub src_ref: SrcRef,
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Init {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Attributes.
    pub attr: ir::Attributes,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    pub statements: Box<[InitStatement]>,
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

#[non_exhaustive]
#[derive(Debug, Clone, From, PartialEq, Hash, Serialize, Deserialize)]
pub enum WorkbenchExpression {
    Invalid,
    Constant(ir::ConstantValue),
    Path(ir::Path),
    Group(ir::Group),
    If(ir::If<WorkbenchExpression>),
    Call(ir::Call<WorkbenchExpression>),
    Marker(Marker),
}

impl SrcReferrer for WorkbenchExpression {
    fn src_ref(&self) -> SrcRef {
        use WorkbenchExpression::*;
        match &self {
            Invalid => SrcRef::none(),
            Constant(literal) => literal.src_ref(),
            Path(name) => name.src_ref(),
            Group(group) => group.src_ref,
            If(if_) => if_.src_ref,
            Call(call) => call.src_ref,
            Marker(marker) => marker.src_ref,
        }
    }
}

impl ir::ExprSpec for WorkbenchExpression {
    type Body = Group;

    fn value(&self) -> Option<&Value> {
        match &self {
            WorkbenchExpression::Constant(constant) => Some(constant.value()),
            _ => None,
        }
    }
}

impl SingleIdentifier for WorkbenchExpression {
    fn single_identifier(&self) -> Option<&microcad_lang_base::Identifier> {
        match self {
            WorkbenchExpression::Path(path) => path.single_identifier(),
            _ => None,
        }
    }
}

impl CastInto<ir::WorkbenchExpression> for ir::ConstantExpression {
    fn cast_into(self: ir::ConstantExpression) -> ir::WorkbenchExpression {
        match self {
            ir::ConstantExpression::Invalid => ir::WorkbenchExpression::Invalid,
            ir::ConstantExpression::Constant(literal) => ir::WorkbenchExpression::Constant(literal),
            ir::ConstantExpression::Path(name) => ir::WorkbenchExpression::Path(name),
            ir::ConstantExpression::Call(call) => ir::WorkbenchExpression::Call(call.cast_into()),
        }
    }
}

/// A workbench signature consists of the workbench kind, a parameter list and statements
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct WorkbenchSignature {
    /// Workbench Kind
    pub kind: ir::WorkbenchKind,
    /// Workbench's building plan.
    pub parameters: ir::ParameterList,
    /// `init`
    pub inits: Box<[ir::Init]>,
}

/// Builder methods for testing
impl WorkbenchSignature {
    /// Create a new Workbench signature
    pub fn new(kind: WorkbenchKind, parameters: impl Into<ir::ParameterList>) -> Self {
        Self {
            kind,
            parameters: parameters.into(),
            inits: Default::default(),
        }
    }

    pub fn with_inits(mut self, inits: impl IntoIterator<Item = ir::Init>) -> Self {
        self.inits = inits.into_iter().collect();
        self
    }
}

/// Workbench definition, e.g `sketch`, `part` or `op`.
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Workbench {
    /// Attributes, combined from Inner and OuterAttributes
    pub attr: ir::Attributes,
    /// Workbench Kind
    pub signature: WorkbenchSignature,
    /// The actual statements to build the Model
    pub statements: Box<[ir::WorkbenchStatement]>,
}
