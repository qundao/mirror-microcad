// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Span;
use crate::ast::{Call, Expression, Identifier, ItemExtras, StatementList, Type};

/// A µcad statements
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Statement {
    /// Workbench statement producing a symbol.
    Workbench(WorkbenchDefinition),
    /// Module statement producing a symbol.
    Module(ModuleDefinition),
    /// Function statement producing a symbol.
    Function(FunctionDefinition),
    /// Use statement producing a symbol.
    Use(UseStatement),
    /// Const statement producing a symbol.
    Const(ConstAssignment),

    Init(InitDefinition),
    Return(Return),
    InnerAttribute(Attribute),
    LocalAssignment(LocalAssignment),
    Property(PropertyAssignment),
    Expression(ExpressionStatement),
    InnerDocComment(Comment),
    Comment(Comment),
    Error(Span),
}

impl Statement {
    /// Get the span for the statement
    pub fn span(&self) -> Span {
        use Statement::*;

        match self {
            Workbench(st) => st.span.clone(),
            Module(st) => st.span.clone(),
            Function(st) => st.span.clone(),
            Use(st) => st.span.clone(),
            Const(st) => st.span.clone(),
            Init(st) => st.span.clone(),
            Return(st) => st.span.clone(),
            InnerAttribute(st) => st.span.clone(),
            LocalAssignment(st) => st.span.clone(),
            Property(st) => st.span.clone(),
            Expression(st) => st.span.clone(),
            InnerDocComment(st) => st.span.clone(),
            Comment(st) => st.span.clone(),
            Error(span) => span.clone(),
        }
    }
}

/// The possible type of workbenches
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum WorkbenchKind {
    Sketch,
    Part,
    Op,
}

/// A definition of a workbench
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct WorkbenchDefinition {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub kind: WorkbenchKind,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub plan: ParameterList,
    pub body: StatementList,
}

/// A definition of a module
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ModuleDefinition {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub body: Option<StatementList>,
}

/// A definition of a function
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct FunctionDefinition {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub arguments: ParameterList,
    pub return_type: Option<Type>,
    pub body: StatementList,
}

/// An init definition for a workbench
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct InitDefinition {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub attributes: Vec<Attribute>,
    pub parameters: ParameterList,
    pub body: StatementList,
}

/// A use statement that imports an item from an external library
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct UseStatement {
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub extras: ItemExtras,
    pub visibility: Option<Visibility>,
    pub name: UseName,
    pub use_as: Option<Identifier>,
}

/// The name of the item being imported
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct UseName {
    pub span: Span,
    pub extras: ItemExtras,
    pub parts: Vec<UseStatementPart>,
}

/// The parts a [`UseName`] consists of, separated by `::`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum UseStatementPart {
    Identifier(Identifier),
    Glob(Span),
    Error(Span),
}

/// A return statement
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Return {
    pub span: Span,
    pub extras: ItemExtras,
    pub value: Option<Expression>,
}

/// A parameter list of a workbench definition or function definition
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ParameterList {
    pub span: Span,
    pub extras: ItemExtras,
    pub parameters: Vec<Parameter>,
}

impl ParameterList {
    pub(crate) fn dummy(span: Span) -> Self {
        ParameterList {
            span,
            extras: ItemExtras::default(),
            parameters: Vec::default(),
        }
    }
}

/// A parameter for a workbench definition or function definition
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Parameter {
    pub span: Span,
    pub extras: ItemExtras,
    pub name: Identifier,
    pub ty: Option<Type>,
    pub default: Option<Expression>,
}

/// An attribute that can be attached to a statement
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Attribute {
    pub span: Span,
    pub is_inner: bool,
    pub extras: ItemExtras,
    pub commands: Vec<AttributeCommand>,
}

/// The contents an an [`Attribute`]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum AttributeCommand {
    Ident(Identifier),
    Assignment(LocalAssignment),
    Call(Call),
}

/// An optional qualifier that can be part of an [`Assignment`]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum AssignmentQualifier {
    Const,
    Prop,
}

/// A local assignment statement: `a = 42;`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct LocalAssignment {
    pub span: Span,
    pub extras: ItemExtras,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub ty: Option<Type>,
    pub value: Box<Expression>,
}

/// A const assignment: `const A = 42` / `pub A = 32`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ConstAssignment {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub ty: Option<Type>,
    pub value: Box<Expression>,
}

/// A property assignment: `prop A = 42`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct PropertyAssignment {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub ty: Option<Type>,
    pub value: Box<Expression>,
}

/// A single- or multi-line comment
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Comment {
    pub span: Span,
    pub lines: Vec<String>,
}

/// An optional visibility modifier that can be art of assignment and module, function and workbench definitions
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Visibility {
    Public,
}

/// A statement containing of a bare expression
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ExpressionStatement {
    pub span: Span,
    pub extras: ItemExtras,
    pub attributes: Vec<Attribute>,
    pub expression: Expression,
}
