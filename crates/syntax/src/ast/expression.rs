use crate::Span;
use crate::ast::{Identifier, ItemExtras, Literal, SingleType, Statement, StringLiteral};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Union,
    Intersect,
    PowerXor,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Equal,
    Near,
    NotEqual,
    And,
    Or,
    Xor,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Union => "|",
            Operator::Intersect => "&",
            Operator::PowerXor => "^",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterEqual => "≥",
            Operator::LessEqual => "≤",
            Operator::Equal => "==",
            Operator::Near => "~",
            Operator::NotEqual => "!=",
            Operator::And => "&",
            Operator::Or => "|",
            Operator::Xor => "^",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Not,
}

impl UnaryOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnaryOperator::Minus => "-",
            UnaryOperator::Plus => "+",
            UnaryOperator::Not => "!",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Tuple(TupleExpression),
    ArrayRange(ArrayRangeExpression),
    ArrayList(ArrayListExpression),
    String(FormatString),
    QualifiedName(QualifiedName),
    Marker(Identifier),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Block(StatementList),
    Call(Call),
    ElementAccess(ElementAccess),
    If(If),
    Error(Span),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Literal(ex) => ex.span(),
            Expression::Tuple(ex) => ex.span.clone(),
            Expression::ArrayRange(ex) => ex.span.clone(),
            Expression::ArrayList(ex) => ex.span.clone(),
            Expression::String(ex) => ex.span.clone(),
            Expression::QualifiedName(ex) => ex.span.clone(),
            Expression::Marker(ex) => ex.span.clone(),
            Expression::BinaryOperation(ex) => ex.span.clone(),
            Expression::UnaryOperation(ex) => ex.span.clone(),
            Expression::Block(ex) => ex.span.clone(),
            Expression::Call(ex) => ex.span.clone(),
            Expression::ElementAccess(ex) => ex.span.clone(),
            Expression::If(ex) => ex.span.clone(),
            Expression::Error(span) => span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FormatString {
    pub span: Span,
    pub extras: ItemExtras,
    pub parts: Vec<StringPart>,
}

#[derive(Debug, PartialEq)]
pub enum StringPart {
    Char(StringCharacter),
    Content(StringLiteral),
    Expression(StringExpression),
}

#[derive(Debug, PartialEq)]
pub struct StringCharacter {
    pub span: Span,
    pub character: char,
}

#[derive(Debug, PartialEq)]
pub struct StringExpression {
    pub span: Span,
    pub extras: ItemExtras,
    pub expression: Box<Expression>,
    pub specification: Box<StringFormatSpecification>,
}

#[derive(Debug, PartialEq)]
pub struct StringFormatSpecification {
    pub span: Span,
    pub precision: Option<Result<u32, (ParseIntError, Span)>>,
    pub width: Option<Result<u32, (ParseIntError, Span)>>,
}

impl StringFormatSpecification {
    pub fn is_some(&self) -> bool {
        self.precision.is_some() || self.width.is_some()
    }
}

#[derive(Debug, PartialEq)]
pub struct TupleItem {
    pub span: Span,
    pub extras: ItemExtras,
    pub name: Option<Identifier>,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct TupleExpression {
    pub span: Span,
    pub extras: ItemExtras,
    pub values: Vec<TupleItem>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayRangeExpression {
    pub span: Span,
    pub extras: ItemExtras,
    pub start: Box<ArrayItem>,
    pub end: Box<ArrayItem>,
    pub ty: Option<SingleType>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayListExpression {
    pub span: Span,
    pub extras: ItemExtras,
    pub items: Vec<ArrayItem>,
    pub ty: Option<SingleType>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayItem {
    pub span: Span,
    pub extras: ItemExtras,
    pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub struct QualifiedName {
    pub span: Span,
    pub extras: ItemExtras,
    pub parts: Vec<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    pub span: Span,
    pub lhs: Box<Expression>,
    pub operation: Operator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct UnaryOperation {
    pub span: Span,
    pub extras: ItemExtras,
    pub operation: UnaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub span: Span,
    pub extras: ItemExtras,
    pub name: QualifiedName,
    pub arguments: ArgumentList,
}

#[derive(Debug, PartialEq)]
pub struct ElementAccess {
    pub span: Span,
    pub value: Box<Expression>,
    pub element: Element,
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Attribute(Identifier),
    Tuple(Identifier),
    Method(Call),
    ArrayElement(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub span: Span,
    pub extras: ItemExtras,
    pub condition: Box<Expression>,
    pub body: StatementList,
    pub next_if: Option<Box<If>>,
    pub else_body: Option<StatementList>,
}

#[derive(Debug, PartialEq)]
pub struct StatementList {
    pub span: Span,
    pub extras: ItemExtras,
    pub statements: Vec<Statement>,
    pub tail: Option<Box<Statement>>,
}

#[derive(Debug, PartialEq)]
pub struct ArgumentList {
    pub span: Span,
    pub extras: ItemExtras,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Unnamed(UnnamedArgument),
    Named(NamedArgument),
}

impl Argument {
    pub fn name(&self) -> Option<&Identifier> {
        match self {
            Argument::Unnamed(_) => None,
            Argument::Named(arg) => Some(&arg.name),
        }
    }

    pub fn value(&self) -> &Expression {
        match self {
            Argument::Unnamed(arg) => &arg.value,
            Argument::Named(arg) => &arg.value,
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            Argument::Unnamed(arg) => &arg.span,
            Argument::Named(arg) => &arg.span,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnnamedArgument {
    pub span: Span,
    pub extras: ItemExtras,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct NamedArgument {
    pub span: Span,
    pub extras: ItemExtras,
    pub name: Identifier,
    pub value: Expression,
}
