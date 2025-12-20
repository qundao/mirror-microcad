use crate::ast::{Expression, Identifier, Literal, QualifiedName, StatementList, Type};
use crate::Span;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Workspace(WorkspaceDefinition),
    Module(ModuleDefinition),
    Function(FunctionDefinition),
    Init(InitDefinition),
    Use(UseStatement),
    Return(Return),
    InnerAttribute(Attribute),
    Assignment(Assignment),
    Expression(Expression),
    Comment(Comment),
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::Workspace(st) => st.span.clone(),
            Statement::Module(st) => st.span.clone(),
            Statement::Function(st) => st.span.clone(),
            Statement::Init(st) => st.span.clone(),
            Statement::Use(st) => st.span.clone(),
            Statement::Return(st) => st.span.clone(),
            Statement::InnerAttribute(st) => st.span.clone(),
            Statement::Assignment(st) => st.span.clone(),
            Statement::Expression(st) => st.span(),
            Statement::Comment(st) => st.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WorkspaceKind {
    Sketch,
    Part,
    Op,
}

#[derive(Debug, PartialEq)]
pub struct WorkspaceDefinition {
    pub span: Span,
    pub kind: WorkspaceKind,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub arguments: Vec<ArgumentDefinition>,
    pub body: StatementList,
}

#[derive(Debug, PartialEq)]
pub struct ModuleDefinition {
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub body: StatementList,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub span: Span,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub arguments: Vec<ArgumentDefinition>,
    pub return_type: Option<Type>,
    pub body: StatementList,
}

#[derive(Debug, PartialEq)]
pub struct InitDefinition {
    pub span: Span,
    pub arguments: Vec<ArgumentDefinition>,
    pub body: StatementList,
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
    pub ty: Option<Identifier>,
    pub default: Option<Literal>,
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub span: Span,
    pub items: Vec<Statement>,
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
    Private,
}
