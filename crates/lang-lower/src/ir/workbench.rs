// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{CastInto, MakeHumanReadable, ir};

use derive_more::{Display, From};
use microcad_lang_base::{Refer, SingleIdentifier, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Identifiable;

pub use microcad_lang_base::element::WorkbenchKind;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Each WorkbenchStatement eventually evals into a [`Models`]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct WorkbenchStatement {
    pub attr: ir::OuterAttributes,
    pub src_ref: SrcRef,
    pub visibility: ir::Visibility, // public = property
    pub keyword_src_ref: SrcRef,
    pub id: Option<ir::Identifier>,
    pub ty: ir::Type,
    pub expression: WorkbenchExpression,
}

impl MakeHumanReadable for WorkbenchStatement {
    fn make_human_readable<U: crate::Unresolver>(&mut self, unresolver: &U) {
        self.attr.make_human_readable(unresolver);
        self.expression.make_human_readable(unresolver);
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct Group {
    pub src_ref: SrcRef,
    pub attr: ir::InnerAttributes,
    pub statements: Box<[WorkbenchStatement]>,
}

impl MakeHumanReadable for Group {
    fn make_human_readable<U: crate::Unresolver>(&mut self, unresolver: &U) {
        self.attr.make_human_readable(unresolver);
        self.statements.make_human_readable(unresolver);
    }
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
    pub statements: Box<[WorkbenchStatement]>,
    /// Source reference
    pub src_ref: SrcRef,
}

impl MakeHumanReadable for Init {
    fn make_human_readable<U: crate::Unresolver>(&mut self, unresolver: &U) {
        self.attr.make_human_readable(unresolver);
        self.parameters.make_human_readable(unresolver);
        self.statements.make_human_readable(unresolver);
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
    Literal(ir::Literal),
    Path(ir::Path),
    Group(ir::Group),
    If(ir::If<WorkbenchExpression>),
    Call(ir::Call<WorkbenchExpression>),
    Marker(Marker),
}

impl MakeHumanReadable for WorkbenchExpression {
    fn make_human_readable<U: crate::Unresolver>(&mut self, unresolver: &U) {
        match self {
            WorkbenchExpression::Path(path) => path.make_human_readable(unresolver),
            WorkbenchExpression::Group(group) => group.make_human_readable(unresolver),
            WorkbenchExpression::If(if_) => if_.make_human_readable(unresolver),
            WorkbenchExpression::Call(call) => call.make_human_readable(unresolver),
            _ => {}
        }
    }
}

impl SrcReferrer for WorkbenchExpression {
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

impl ir::ExprSpec for WorkbenchExpression {
    type Body = Group;
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
            ir::ConstantExpression::Literal(literal) => ir::WorkbenchExpression::Literal(literal),
            ir::ConstantExpression::Path(name) => ir::WorkbenchExpression::Path(name),
            ir::ConstantExpression::Call(call) => ir::WorkbenchExpression::Call(call.cast_into()),
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

impl MakeHumanReadable for WorkbenchItems {
    fn make_human_readable<U: crate::Unresolver>(&mut self, unresolver: &U) {
        self.aliases.make_human_readable(unresolver);
        self.constants.make_human_readable(unresolver);
        self.functions.make_human_readable(unresolver);
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

impl MakeHumanReadable for Workbench {
    fn make_human_readable<U: crate::Unresolver>(&mut self, unresolver: &U) {
        self.outer_attr.make_human_readable(unresolver);
        self.parameters.make_human_readable(unresolver);
        self.inner_attr.make_human_readable(unresolver);
        self.inits.make_human_readable(unresolver);
        self.items.make_human_readable(unresolver);
        self.statements.make_human_readable(unresolver);
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
