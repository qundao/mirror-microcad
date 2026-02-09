// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Span;
use crate::ast::{Identifier, ItemExtras, Literal, SingleType, Statement, StringLiteral};
use std::num::ParseIntError;

/// An operator for binary operators
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
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
    /// Get the symbolic representation for the operator
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

/// An operator for unary operators
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Not,
}

impl UnaryOperator {
    /// Get the symbolic representation for the operator
    pub fn as_str(&self) -> &'static str {
        match self {
            UnaryOperator::Minus => "-",
            UnaryOperator::Plus => "+",
            UnaryOperator::Not => "!",
        }
    }
}

/// Any expression.
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
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
    /// Get the source span for the identifier
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

/// A string containing a format expression
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct FormatString {
    pub span: Span,
    pub extras: ItemExtras,
    pub parts: Vec<StringPart>,
}

/// A part of a [`FormatString`]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum StringPart {
    Char(StringCharacter),
    Content(StringLiteral),
    Expression(StringExpression),
}

/// A single character that is part of a [`FormatString`]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct StringCharacter {
    pub span: Span,
    pub character: char,
}

/// A format expression that is part of a [`FormatString`]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct StringExpression {
    pub span: Span,
    pub extras: ItemExtras,
    pub expression: Box<Expression>,
    pub specification: Box<StringFormatSpecification>,
}

/// The format specification for a [`StringExpression`], specifying the with and precision for number formatting
///
/// All parts of the specification are optional
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct StringFormatSpecification {
    pub span: Span,
    pub precision: Option<Result<u32, (ParseIntError, Span)>>,
    pub width: Option<Result<u32, (ParseIntError, Span)>>,
}

impl StringFormatSpecification {
    /// Check if an part of the specification is specified
    pub fn is_some(&self) -> bool {
        self.precision.is_some() || self.width.is_some()
    }
}

/// An item that is part of a tuple expression
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct TupleItem {
    pub span: Span,
    pub extras: ItemExtras,
    pub name: Option<Identifier>,
    pub value: Expression,
}

/// A tuple expression, a fixed size set of items that don't need to be the same type
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct TupleExpression {
    pub span: Span,
    pub extras: ItemExtras,
    pub values: Vec<TupleItem>,
}

/// An array range, containing all values from the start value (inclusive) till then end value (exclusive)
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ArrayRangeExpression {
    pub span: Span,
    pub extras: ItemExtras,
    pub start: Box<ArrayItem>,
    pub end: Box<ArrayItem>,
    pub ty: Option<SingleType>,
}

/// An array specified as a list of items
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ArrayListExpression {
    pub span: Span,
    pub extras: ItemExtras,
    pub items: Vec<ArrayItem>,
    pub ty: Option<SingleType>,
}

/// An item that can be part of an array expression
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ArrayItem {
    pub span: Span,
    pub extras: ItemExtras,
    pub expression: Expression,
}

/// A qualified name, containing one or more [`Literal`]s seperated by `::`
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct QualifiedName {
    pub span: Span,
    pub extras: ItemExtras,
    pub parts: Vec<Identifier>,
}

/// A binary operation
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct BinaryOperation {
    pub span: Span,
    pub lhs: Box<Expression>,
    pub operation: Operator,
    pub rhs: Box<Expression>,
}

/// A unary operation
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct UnaryOperation {
    pub span: Span,
    pub extras: ItemExtras,
    pub operation: UnaryOperator,
    pub rhs: Box<Expression>,
}

/// A function call
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Call {
    pub span: Span,
    pub extras: ItemExtras,
    pub name: QualifiedName,
    pub arguments: ArgumentList,
}

/// An expression that access an element from another expression.
///
/// Either accessing an array or tuple item, accessing an attribute of a value or a method call.
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ElementAccess {
    pub span: Span,
    pub value: Box<Expression>,
    pub element: Element,
}

/// The possible element access types
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Element {
    Attribute(Identifier),
    Tuple(Identifier),
    Method(Call),
    ArrayElement(Box<Expression>),
}

/// An if expression, can be used as either a statement or expression
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct If {
    pub span: Span,
    pub extras: ItemExtras,
    pub condition: Box<Expression>,
    pub body: StatementList,
    pub next_if: Option<Box<If>>,
    pub else_body: Option<StatementList>,
}

/// A list of statements, with an optional "tail" expression
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct StatementList {
    pub span: Span,
    pub extras: ItemExtras,
    pub statements: Vec<Statement>,
    pub tail: Option<Box<Statement>>,
}

/// A list of arguments to a function call
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ArgumentList {
    pub span: Span,
    pub extras: ItemExtras,
    pub arguments: Vec<Argument>,
}

/// A function argument that is part of an [`ArgumentList`]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Argument {
    Unnamed(UnnamedArgument),
    Named(NamedArgument),
}

impl Argument {
    /// The name of the argument, if specified
    pub fn name(&self) -> Option<&Identifier> {
        match self {
            Argument::Unnamed(_) => None,
            Argument::Named(arg) => Some(&arg.name),
        }
    }

    /// The value of the argument
    pub fn value(&self) -> &Expression {
        match self {
            Argument::Unnamed(arg) => &arg.value,
            Argument::Named(arg) => &arg.value,
        }
    }

    /// The span of the argument
    pub fn span(&self) -> &Span {
        match self {
            Argument::Unnamed(arg) => &arg.span,
            Argument::Named(arg) => &arg.span,
        }
    }
}

/// An argument without specified name
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct UnnamedArgument {
    pub span: Span,
    pub extras: ItemExtras,
    pub value: Expression,
}

/// An argument with a specified name
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub struct NamedArgument {
    pub span: Span,
    pub extras: ItemExtras,
    pub name: Identifier,
    pub value: Expression,
}
