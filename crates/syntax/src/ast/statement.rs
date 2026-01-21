use crate::ast::{Call, Expression, Identifier, Literal, QualifiedName, StatementList, Type};
use crate::Span;

#[derive(Debug, PartialEq)]
pub enum InitStatement {
    Function(FunctionDefinition),
    Init(InitDefinition),
    Use(UseStatement),
    Assignment(Assignment),
    Comment(Comment),
}

impl InitStatement {
    pub fn span(&self) -> Span {
        match self {
            Self::Function(st) => st.span.clone(),
            Self::Init(st) => st.span.clone(),
            Self::Use(st) => st.span.clone(),
            Self::Assignment(st) => st.span.clone(),
            Self::Comment(st) => st.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InitializerStatement {
    Use(UseStatement),
    Assignment(Assignment),
    Comment(Comment),
}

impl InitializerStatement {
    pub fn span(&self) -> Span {
        match self {
            Self::Use(st) => st.span.clone(),
            Self::Assignment(st) => st.span.clone(),
            Self::Comment(st) => st.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BuildingStatement {
    Function(FunctionDefinition),
    Use(UseStatement),
    Assignment(Assignment),
    Expression(Expression),
    Comment(Comment),
}

impl BuildingStatement {
    pub fn span(&self) -> Span {
        match self {
            Self::Function(st) => st.span.clone(),
            Self::Use(st) => st.span.clone(),
            Self::Assignment(st) => st.span.clone(),
            Self::Expression(st) => st.span(),
            Self::Comment(st) => st.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ModuleStatement {
    Workbench(WorkbenchDefinition),
    Module(ModuleDefinition),
    Function(FunctionDefinition),
    Use(UseStatement),
    Assignment(Assignment),
    Comment(Comment),
}

impl ModuleStatement {
    pub fn span(&self) -> Span {
        match self {
            Self::Workbench(st) => st.span.clone(),
            Self::Module(st) => st.span.clone(),
            Self::Function(st) => st.span.clone(),
            Self::Use(st) => st.span.clone(),
            Self::Assignment(st) => st.span.clone(),
            Self::Comment(st) => st.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SourceFileStatement {
    Workbench(WorkbenchDefinition),
    Module(ModuleDefinition),
    Function(FunctionDefinition),
    Use(UseStatement),
    InnerAttribute(Attribute),
    Assignment(Assignment),
    Expression(Expression),
    Comment(Comment),
}

impl SourceFileStatement {
    pub fn span(&self) -> Span {
        match self {
            Self::Workbench(st) => st.span.clone(),
            Self::Module(st) => st.span.clone(),
            Self::Function(st) => st.span.clone(),
            Self::Use(st) => st.span.clone(),
            Self::InnerAttribute(st) => st.span.clone(),
            Self::Assignment(st) => st.span.clone(),
            Self::Expression(st) => st.span(),
            Self::Comment(st) => st.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FunctionStatement {
    Use(UseStatement),
    Return(Return),
    Assignment(Assignment),
    Expression(Expression),
    Comment(Comment),
}

impl FunctionStatement {
    pub fn span(&self) -> Span {
        match self {
            Self::Use(st) => st.span.clone(),
            Self::Return(st) => st.span.clone(),
            Self::Assignment(st) => st.span.clone(),
            Self::Expression(st) => st.span(),
            Self::Comment(st) => st.span.clone(),
        }
    }
}

pub type IfElseStatement = FunctionStatement;

#[derive(Debug, PartialEq)]
pub enum ExpressionStatement {
    Use(UseStatement),
    Assignment(Assignment),
    Expression(Expression),
    Comment(Comment),
}

impl ExpressionStatement {
    pub fn span(&self) -> Span {
        match self {
            Self::Use(st) => st.span.clone(),
            Self::Assignment(st) => st.span.clone(),
            Self::Expression(st) => st.span(),
            Self::Comment(st) => st.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WorkbenchKind {
    Sketch,
    Part,
    Op,
}

#[derive(Debug, PartialEq)]
pub struct WorkbenchDefinition {
    pub span: Span,
    pub kind: WorkbenchKind,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub arguments: Vec<ArgumentDefinition>,
    pub init: StatementList<InitStatement>,
    pub initializers: Vec<InitDefinition>,
    pub body: StatementList<BuildingStatement>,
}

#[derive(Debug, PartialEq)]
pub struct ModuleDefinition {
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub body: StatementList<ModuleStatement>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub span: Span,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub arguments: Vec<ArgumentDefinition>,
    pub return_type: Option<Type>,
    pub body: StatementList<FunctionStatement>,
}

#[derive(Debug, PartialEq)]
pub struct InitDefinition {
    pub span: Span,
    pub arguments: Vec<ArgumentDefinition>,
    pub body: StatementList<InitStatement>,
}

#[derive(Debug, PartialEq)]
pub struct UseStatement {
    pub span: Span,
    pub name: QualifiedName,
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub span: Span,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ArgumentDefinition {
    pub span: Span,
    pub name: Identifier,
    pub ty: Option<Type>,
    pub default: Option<Literal>,
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub span: Span,
    pub name: Identifier,
    pub command: AttributeCommand,
}

#[derive(Debug, PartialEq)]
pub enum AttributeCommand {
    Ident(QualifiedName),
    Assignment(Assignment),
    Call(Call),
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub span: Span,
    pub name: Identifier,
    pub ty: Option<Type>,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub span: Span,
    pub comment: String,
}

#[derive(Debug, PartialEq)]
pub enum Visibility {
    Public,
}
