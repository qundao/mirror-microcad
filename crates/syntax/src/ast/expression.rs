use crate::ast::*;
use crate::Span;
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

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    FormatString(FormatString),
    Tuple(TupleExpression),
    ArrayRange(ArrayRangeExpression),
    ArrayList(ArrayListExpression),
    String(FormatString),
    QualifiedName(QualifiedName),
    Marker(Identifier),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Block(StatementList<ExpressionStatement>),
    Call(Call),
    If(If),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Literal(ex) => ex.span(),
            Expression::FormatString(ex) => ex.span.clone(),
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
            Expression::If(ex) => ex.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FormatString {
    pub span: Span,
    pub parts: Vec<StringPart>,
}

#[derive(Debug, PartialEq)]
pub enum StringPart {
    Char(char),
    Content(StringContent),
    Expression(StringExpression),
}

#[derive(Debug, PartialEq)]
pub struct StringExpression {
    pub span: Span,
    pub expression: Expression,
    pub accuracy: Option<Result<usize, (ParseIntError, Span)>>,
    pub width: Option<Result<usize, (ParseIntError, Span)>>,
}

#[derive(Debug, PartialEq)]
pub struct TupleExpression {
    pub span: Span,
    pub values: Vec<(Option<Identifier>, Expression)>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayRangeExpression {
    pub span: Span,
    pub start: Box<Expression>,
    pub end: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayListExpression {
    pub span: Span,
    pub items: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct QualifiedName {
    pub span: Span,
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
    pub operation: UnaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub span: Span,
    pub name: QualifiedName,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub span: Span,
    pub condition: Box<Expression>,
    pub body: Box<StatementList<IfElseStatement>>,
    pub next_if: Option<Box<If>>,
    pub else_body: Box<Option<IfElseStatement>>,
}

#[derive(Debug, PartialEq)]
pub struct StatementList<S> {
    pub span: Span,
    pub statements: Vec<S>,
    pub tail: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Positional(PositionArgument),
    Named(NamedArgument),
}

#[derive(Debug, PartialEq)]
pub struct PositionArgument {
    pub span: Span,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct NamedArgument {
    pub span: Span,
    pub name: Identifier,
    pub value: Expression,
}
