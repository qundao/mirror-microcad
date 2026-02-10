// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ast::{Call, Expression, Identifier, ItemExtras, StatementList, Type};
use crate::Span;

/// A µcad statements
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Statement {
    Workbench(WorkbenchDefinition),
    Module(ModuleDefinition),
    Function(FunctionDefinition),
    Init(InitDefinition),
    Use(UseStatement),
    Return(Return),
    InnerAttribute(Attribute),
    Assignment(Assignment),
    Expression(ExpressionStatement),
    Comment(Comment),
    Error(Span),
}

impl Statement {
    /// Get the span for the statement
    pub fn span(&self) -> Span {
        match self {
            Statement::Workbench(st) => st.span.clone(),
            Statement::Module(st) => st.span.clone(),
            Statement::Function(st) => st.span.clone(),
            Statement::Init(st) => st.span.clone(),
            Statement::Use(st) => st.span.clone(),
            Statement::Return(st) => st.span.clone(),
            Statement::InnerAttribute(st) => st.span.clone(),
            Statement::Assignment(st) => st.span.clone(),
            Statement::Expression(st) => st.span.clone(),
            Statement::Comment(st) => st.span.clone(),
            Statement::Error(span) => span.clone(),
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
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub kind: WorkbenchKind,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub arguments: ArgumentsDefinition,
    pub body: StatementList,
}

/// A definition of a module
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ModuleDefinition {
    pub span: Span,
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
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub arguments: ArgumentsDefinition,
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
    pub arguments: ArgumentsDefinition,
    pub body: StatementList,
}

/// A use statement that imports an item from an external library
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct UseStatement {
    pub span: Span,
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
}

/// A return statement
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Return {
    pub span: Span,
    pub extras: ItemExtras,
    pub value: Option<Expression>,
}

/// A definition of the arguments of a workbench definition or function definition
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ArgumentsDefinition {
    pub span: Span,
    pub extras: ItemExtras,
    pub arguments: Vec<ArgumentDefinition>,
}

/// A definition of a single of a workbench definition or function definition
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ArgumentDefinition {
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
    Assignment(Assignment),
    Call(Call),
}

/// An optional qualifier that can be part of an [`Assignment`]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum AssignmentQualifier {
    Const,
    Prop,
}

/// An assignment statement
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Assignment {
    pub span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub qualifier: Option<AssignmentQualifier>,
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
