use indexmap::IndexMap;
use crate::ast::{Expression, Identifier, Type};
use crate::Span;

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(StringContent),
    FormatString(FormatString),
    Bool(BoolLiteral),
    Integer(IntegerLiteral),
    Quantity(QuantityLiteral),
    Tuple(TupleLiteral),
    NamedTuple(NamedTupleLiteral),
}

impl Literal {
    pub fn span(&self) -> Span {
        match self {
            Literal::String(lit) => lit.span.clone(),
            Literal::FormatString(lit) => lit.span.clone(),
            Literal::Bool(lit) => lit.span.clone(),
            Literal::Integer(lit) => lit.span.clone(),
            Literal::Quantity(lit) => lit.span.clone(),
            Literal::Tuple(lit) => lit.span.clone(),
            Literal::NamedTuple(lit) => lit.span.clone(),
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
    Content(StringContent),
    Expression(StringExpression),
}

#[derive(Debug, PartialEq)]
pub struct StringContent {
    pub span: Span,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct StringExpression {
    pub span: Span,
    pub expression: Expression,
    pub accuracy: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub struct BoolLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub span: Span,
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub struct QuantityLiteral {
    pub span: Span,
    pub value: f64,
    pub ty: Option<Type>,
}

#[derive(Debug, PartialEq)]
pub struct TupleLiteral {
    pub span: Span,
    pub values: Vec<Literal>,
}

#[derive(Debug, PartialEq)]
pub struct NamedTupleLiteral {
    pub span: Span,
    pub values: IndexMap<Identifier, Literal>,
}