// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{CastInto, ir};

use derive_more::{Display, From};
use microcad_lang_base::{Identifier, SingleIdentifier, SrcRef, SrcReferrer, element::Visibility};

pub use microcad_lang_base::element::WorkbenchKind;
use microcad_lang_types::{FunctionType, Value};
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
    /// Construct an expression that will eventually produce a model, `__mu::geo2d::Circle(radius = 4.0mm)`.
    pub fn expr(expr: impl Into<WorkbenchExpression>) -> Self {
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

    /// Construct a property, `prop a = 42mm`.
    pub fn prop(name: impl AsRef<str>, expr: impl Into<WorkbenchExpression>) -> Self {
        Self {
            name: Some(Identifier::from(name.as_ref())),
            attr: Default::default(),
            src_ref: Default::default(),
            visibility: Visibility::Public,
            keyword_src_ref: Default::default(),
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

/// Builder methods for testing
impl InitStatement {
    pub fn new(name: impl AsRef<str>, expr: impl Into<WorkbenchExpression>) -> Self {
        Self {
            name: Identifier::from(name.as_ref()),
            expression: expr.into(),
            src_ref: SrcRef::none(),
        }
    }

    pub fn with_src_ref(mut self, src_ref: SrcRef) -> Self {
        self.src_ref = src_ref;
        self
    }
}

/// A workbench initializer that can be called.
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Init {
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    pub statements: Box<[InitStatement]>,
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Attributes.
    pub attr: ir::Attributes,
    /// Source reference
    pub src_ref: SrcRef,
}

/// Builder methods for testing
impl Init {
    /// Construct new initializer
    pub fn default_init(parameters: impl Into<ir::ParameterList>) -> Self {
        let parameters = parameters.into();
        Self {
            parameters: parameters.clone(),
            statements: Default::default(),
            keyword_ref: Default::default(),
            attr: Default::default(),
            src_ref: Default::default(),
        }
        .with_statements(parameters.iter().map(|parameter| {
            ir::InitStatement::new(
                parameter.id.clone(),
                ir::Path::Resolved(microcad_lang_base::SymbolId::Local(
                    parameter.id.id().clone(),
                )),
            )
        }))
    }

    /// Add statements to this initializer
    pub fn with_statements(mut self, statements: impl IntoIterator<Item = InitStatement>) -> Self {
        self.statements = statements
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self
    }
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
        let parameters = parameters.into();
        Self {
            kind,
            inits: vec![Init::default_init(parameters.clone())].into_boxed_slice(), // Default init
            parameters,
        }
    }

    pub fn with_inits(mut self, inits: impl IntoIterator<Item = ir::Init>) -> Self {
        // 1. New inits go at the front (indices 0..N)
        let mut combined: Vec<_> = inits.into_iter().collect();

        // 2. Original self.inits are placed AFTER the new inits
        combined.extend(self.inits.into_vec());

        self.inits = combined.into_boxed_slice();
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
