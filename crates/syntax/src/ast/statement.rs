use crate::Span;
use crate::ast::{Call, Expression, Identifier, Literal, QualifiedName, StatementList, Type};

#[derive(Debug, PartialEq)]
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
    Error,
}

impl Statement {
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
            Statement::Error => 0..0,
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
pub struct WorkbenchDefinition {
    pub span: Span,
    pub doc: Option<Comment>,
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
    pub doc: Option<Comment>,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub body: Option<StatementList>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub span: Span,
    pub doc: Option<Comment>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub arguments: Vec<ArgumentDefinition>,
    pub return_type: Option<Type>,
    pub body: StatementList,
}

#[derive(Debug, PartialEq)]
pub struct InitDefinition {
    pub span: Span,
    pub doc: Option<Comment>,
    pub arguments: Vec<ArgumentDefinition>,
    pub body: StatementList,
}

#[derive(Debug, PartialEq)]
pub struct UseStatement {
    pub span: Span,
    pub visibility: Option<Visibility>,
    pub name: UseName,
    pub use_as: Option<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct UseName {
    pub span: Span,
    pub parts: Vec<UseStatementPart>,
}

#[derive(Debug, PartialEq)]
pub enum UseStatementPart {
    Identifier(Identifier),
    Glob(Span),
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
    pub command: AttributeCommand,
}

#[derive(Debug, PartialEq)]
pub enum AttributeCommand {
    Ident(QualifiedName),
    Assignment(Assignment),
    Call(Call),
}

#[derive(Debug, PartialEq)]
pub enum AssigmentQualifier {
    Const,
    Prop
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub span: Span,
    pub doc: Option<Comment>,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub qualifier: Option<AssigmentQualifier>,
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

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub span: Span,
    pub attributes: Vec<Attribute>,
    pub expression: Expression,
}