use crate::ast::{Call, Expression, Identifier, ItemExtras, StatementList, Type};
use crate::Span;

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
    Error(Span),
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
            Statement::Error(span) => span.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WorkbenchKind {
    Sketch,
    Part,
    Op,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct ModuleDefinition {
    pub span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Identifier,
    pub body: Option<StatementList>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct InitDefinition {
    pub span: Span,
    pub extras: ItemExtras,
    pub doc: Option<Comment>,
    pub arguments: ArgumentsDefinition,
    pub body: StatementList,
}

#[derive(Debug, PartialEq)]
pub struct UseStatement {
    pub span: Span,
    pub extras: ItemExtras,
    pub visibility: Option<Visibility>,
    pub name: UseName,
    pub use_as: Option<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct UseName {
    pub span: Span,
    pub extras: ItemExtras,
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
    pub extras: ItemExtras,
    pub value: Option<Expression>,
}
#[derive(Debug, PartialEq)]
pub struct ArgumentsDefinition {
    pub span: Span,
    pub extras: ItemExtras,
    pub arguments: Vec<ArgumentDefinition>,
}

#[derive(Debug, PartialEq)]
pub struct ArgumentDefinition {
    pub span: Span,
    pub extras: ItemExtras,
    pub name: Identifier,
    pub ty: Option<Type>,
    pub default: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub span: Span,
    pub is_inner: bool,
    pub extras: ItemExtras,
    pub commands: Vec<AttributeCommand>,
}

#[derive(Debug, PartialEq)]
pub enum AttributeCommand {
    Ident(Identifier),
    Assignment(Assignment),
    Call(Call),
}

#[derive(Debug, PartialEq)]
pub enum AssignmentQualifier {
    Const,
    Prop,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub span: Span,
    pub lines: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Visibility {
    Public,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub span: Span,
    pub extras: ItemExtras,
    pub attributes: Vec<Attribute>,
    pub expression: Expression,
}
