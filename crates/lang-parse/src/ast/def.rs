// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! These definitions in the AST will eventually become symbols

use crate::ast::{
    Attribute, Body, DocBlock, Expression, Identifier, ItemExtras, ParameterList, Span, Type,
    Visibility,
};

/// The possible type of workbenches
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WorkbenchKind {
    /// `sketch`
    Sketch,
    /// `part`
    Part,
    /// `op`
    Op,
}

impl std::fmt::Display for WorkbenchKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                WorkbenchKind::Sketch => "sketch",
                WorkbenchKind::Part => "part",
                WorkbenchKind::Op => "op",
            }
        )
    }
}

/// A definition of a workbench
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Workbench {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: DocBlock,
    pub kind: WorkbenchKind,
    pub attr: Vec<Attribute>,
    pub vis: Option<Visibility>,
    pub id: Identifier,
    pub parameters: ParameterList,
    pub body: Body,
}

/// A definition of a module
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct InlineModule {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: DocBlock,
    pub attr: Vec<Attribute>,
    pub vis: Option<Visibility>,
    pub id: Identifier,
    pub body: Body,
}

/// A definition of a module
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct FileModule {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: DocBlock,
    pub attr: Vec<Attribute>,
    pub vis: Option<Visibility>,
    pub id: Identifier,
}

/// A definition of a function
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Function {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: DocBlock,
    pub attr: Vec<Attribute>,
    pub vis: Option<Visibility>,
    pub id: Identifier,
    pub parameters: ParameterList,
    pub return_type: Option<Type>,
    pub body: Body,
}

/// A use definition will become an alias or a wildcard.
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Use {
    pub span: Span,
    pub attr: Vec<Attribute>,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub vis: Option<Visibility>,
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

/// A const assignment: `const A = 42` / `pub A = 32`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Constant {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ItemExtras,
    pub doc: DocBlock,
    pub attr: Vec<Attribute>,
    pub vis: Option<Visibility>,
    pub id: Identifier,
    pub ty: Option<Type>,
    pub expr: Box<Expression>,
}
