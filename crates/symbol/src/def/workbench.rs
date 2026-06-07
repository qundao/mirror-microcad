// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_types::{Type, Value};

use crate::def::{
    ParameterValueList, attribute,
    expression::{
        ArrayExpression, BinaryOp, Call, ElementAccess, FormatString, If, TupleExpression, UnaryOp,
    },
};

pub use microcad_lang_lower::hir::WorkbenchKind;

#[derive(Debug)]
pub struct Group(pub Vec<WorkbenchStatement>);

#[derive(Debug)]
pub struct InputPlaceholder;

#[derive(Debug, derive_more::From)]
pub enum WorkbenchExpression {
    Value(Value),
    FormatString(FormatString),
    Array(ArrayExpression<WorkbenchExpression>),
    Tuple(TupleExpression<WorkbenchExpression>),
    Group(Group),
    If(If<WorkbenchExpression, Group>),
    Call(Call<WorkbenchExpression>),
    Input(InputPlaceholder),
    BinaryOp(BinaryOp<WorkbenchExpression>),
    UnaryOp(UnaryOp<WorkbenchExpression>),
    ElementAccess(ElementAccess<WorkbenchExpression>),
}

#[derive(Debug)]
pub struct PropertyStatement {
    pub doc: attribute::DocBlock,
    pub attr: attribute::PropertyAttributes,
    pub id: Identifier,
    pub ty: Type,
    pub expression: WorkbenchExpression,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub metadata: attribute::Metadata,
    pub commands: Vec<attribute::Command>,
    pub expression: WorkbenchExpression,
    pub src_ref: SrcRef,
}

#[derive(Debug)]
pub struct LocalAssignment {
    pub metadata: attribute::Metadata,
    pub commands: Vec<attribute::Command>,
    pub id: Identifier,
    pub expression: WorkbenchExpression,
    pub src_ref: SrcRef,
}

#[derive(Debug)]
pub enum WorkbenchStatement {
    Local(LocalAssignment),
    Expression(ExpressionStatement),
    Property(PropertyStatement),
}

#[derive(Debug)]
pub struct Workbench {
    pub metadata: attribute::Metadata,
    pub kind: WorkbenchKind,
    pub parameters: ParameterValueList,
    pub inits: Vec<Init>,
    pub script: Vec<WorkbenchStatement>,
}

#[derive(Debug)]
pub struct InitAttributes;

#[derive(Debug)]
pub struct Init {
    pub doc: attribute::DocBlock,

    pub attr: InitAttributes,

    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Parameter list for this init definition
    pub parameters: ParameterValueList,
    /// Body if the init definition
    pub assignments: Vec<LocalAssignment>,
}
