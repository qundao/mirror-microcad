// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ast;
use crate::ast::Span;

/// An inner doc block
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct InnerDocComment {
    pub span: Span,
    pub line: String,
}

/// A µcad statement.
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// Workbench statement: `part Foo() { ... }`
    Workbench(WorkbenchDefinition),
    /// Inline Module: `mod foo { ... }`
    InlineModule(InlineModule),
    /// File Module: `mod foo;`
    FileModule(FileModule),
    /// Function statement: `fn bar() { ... }`
    Function(FunctionDefinition),
    /// Use statement: `use foo::bar;`
    Use(UseStatement),
    /// Const definition: `const FOO = 42mm`
    Const(ConstAssignment),
    /// Init definition: `init() { ... }`
    Init(InitDefinition),
    /// Return statement: `return 23mm;`
    Return(Return),
    /// Inner attribute: `#![...]`
    InnerAttribute(Attribute),
    /// Inner documentation: `//! Doc comment`
    InnerDocComment(InnerDocComment),
    /// Local assignment: `foo = bar;`
    LocalAssignment(LocalAssignment),
    /// Property: `prop bar = 42mm;`
    Property(PropertyAssignment),
    /// Expression statement: `foo | bar;`
    Expression(ExpressionStatement),
    /// Any error occured during parsing.
    Error(Span),
}

impl Statement {
    /// Get the span for the statement
    pub fn span(&self) -> Span {
        use Statement::*;

        match self {
            Workbench(st) => st.span.clone(),
            InlineModule(st) => st.span.clone(),
            FileModule(st) => st.span.clone(),
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
            Error(span) => span.clone(),
        }
    }

    /// Test if statement is supposed to end with a semicolon.
    pub fn ends_with_semicolon(&self) -> bool {
        match self {
            Statement::Workbench(_) => false,
            Statement::InlineModule(_) => false,
            Statement::Function(_) => false,
            Statement::InnerAttribute(_) => false,
            Statement::InnerDocComment(_) => false,
            Statement::Init(_) => false,
            Statement::Error(_) => false,

            Statement::Use(_) => true,
            Statement::Const(_) => true,
            Statement::Return(_) => true,
            Statement::FileModule(_) => true,
            Statement::LocalAssignment(_) => true,
            Statement::Property(_) => true,
            Statement::Expression(e) => !matches!(
                &e.expression,
                ast::Expression::Body(_) | ast::Expression::If(_)
            ),
        }
    }
}

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
pub struct WorkbenchDefinition {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub doc: DocBlock,
    pub kind: WorkbenchKind,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: ast::Identifier,
    pub plan: ParameterList,
    pub body: ast::Body,
}

/// A definition of a module
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct InlineModule {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub doc: DocBlock,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: ast::Identifier,
    pub body: ast::Body,
}

/// A definition of a module
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct FileModule {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub doc: DocBlock,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: ast::Identifier,
}

/// A definition of a function
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct FunctionDefinition {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub doc: DocBlock,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: ast::Identifier,
    pub parameters: ParameterList,
    pub return_type: Option<ast::Type>,
    pub body: ast::Body,
}

/// An init definition for a workbench
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct InitDefinition {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub doc: DocBlock,
    pub attributes: Vec<Attribute>,
    pub parameters: ParameterList,
    pub body: ast::Body,
}

/// A use statement that imports an item from an external library
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct UseStatement {
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub visibility: Option<Visibility>,
    pub name: UseName,
    pub use_as: Option<ast::Identifier>,
}

/// The name of the item being imported
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct UseName {
    pub span: Span,
    pub extras: ast::ItemExtras,
    pub parts: Vec<UseStatementPart>,
}

/// The parts a [`UseName`] consists of, separated by `::`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum UseStatementPart {
    Identifier(ast::Identifier),
    Glob(Span),
    Error(Span),
}

/// A return statement
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Return {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub value: Option<ast::Expression>,
}

/// A parameter list of a workbench definition or function definition
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ParameterList {
    pub span: Span,
    pub extras: ast::ItemExtras,
    pub parameters: Vec<Parameter>,
}

impl ast::Dummy for ParameterList {
    fn dummy(span: Span) -> Self {
        Self {
            span,
            extras: ast::ItemExtras::default(),
            parameters: Vec::default(),
        }
    }
}

/// A parameter for a workbench definition or function definition
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Parameter {
    pub span: Span,
    pub extras: ast::ItemExtras,
    pub doc: ast::DocBlock,
    pub attributes: Vec<Attribute>,
    pub name: ast::Identifier,
    pub ty: Option<ast::Type>,
    pub default: Option<ast::Expression>,
}

/// An attribute that can be attached to a statement
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Attribute {
    pub span: Span,
    pub is_inner: bool,
    pub extras: ast::ItemExtras,
    pub commands: Vec<AttributeCommand>,
}

/// The contents an an [`Attribute`]
#[derive(Debug, PartialEq)]
pub enum AttributeCommand {
    /// A single identifier: `#[deprecated]`
    Ident(ast::Identifier),
    /// A meta data assignent: `#[color = RED]`
    Assignment(LocalAssignment),
    /// A call: `#[export("file.svg")`
    Call(ast::Call),
}

/// A local assignment: `a = 42`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct LocalAssignment {
    pub span: Span,
    pub extras: ast::ItemExtras,
    pub attributes: Vec<Attribute>,
    pub name: ast::Identifier,
    pub ty: Option<ast::Type>,
    pub value: Box<ast::Expression>,
}

/// A const assignment: `const A = 42` / `pub A = 32`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ConstAssignment {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub doc: DocBlock,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: ast::Identifier,
    pub ty: Option<ast::Type>,
    pub value: Box<ast::Expression>,
}

/// A property assignment: `prop a = 42`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct PropertyAssignment {
    pub span: Span,
    pub keyword_span: Span,
    pub extras: ast::ItemExtras,
    pub doc: DocBlock,
    pub attributes: Vec<Attribute>,
    pub name: ast::Identifier,
    pub ty: Option<ast::Type>,
    pub value: Box<ast::Expression>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum CommentInner {
    // A list of single line comments starting with `//`.
    SingleLine(String),
    // Comments embraced with `/* ... */`.
    MultiLine(String),
}

/// A single- or multi-line comment
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct Comment {
    pub span: Span,
    pub inner: CommentInner,
}

/// Lines of inner or outer doc block including prefix `///`/`//!`.
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct DocBlock {
    pub span: Span,
    pub lines: Vec<String>,
}

/// An optional visibility modifier
///
/// it can be part of constant, module, function or workbench definitions.
#[derive(Debug, PartialEq)]
pub enum Visibility {
    /// `pub`
    Public,
}

/// A statement containing of a bare expression
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ExpressionStatement {
    pub span: Span,
    pub extras: ast::ItemExtras,
    pub attributes: Vec<Attribute>,
    pub expression: ast::Expression,
}

/// A list of statements, with optional trailing whitespace kept and an optional "tail" expression
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct StatementList {
    pub span: Span,
    pub extras: ast::ItemExtras,
    pub statements: Vec<(Statement, ast::TrailingExtras)>,
    pub tail: Option<Box<ExpressionStatement>>,
}

impl ast::Dummy for StatementList {
    fn dummy(span: Span) -> Self {
        Self {
            span,
            extras: ast::ItemExtras::default(),
            statements: Vec::default(),
            tail: None,
        }
    }
}
