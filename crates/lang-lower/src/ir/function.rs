// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::{CastInto, ir, lower::LowerName};

use derive_more::From;
use microcad_lang_base::{IsDefault, Refer, SingleIdentifier, SrcRef, SrcReferrer};
use serde::{Deserialize, Serialize};

/// Parameters and return type of a function
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct FunctionSignature<Name: ir::NameSpec = ir::SymbolPath> {
    /// Function's parameters
    pub parameters: ir::ParameterList<Name>,
    /// Function's return type
    pub return_type: Option<ir::TypeAnnotation>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<Name: ir::NameSpec> std::fmt::Display for FunctionSignature<Name>
where
    Name: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}){}",
            self.parameters,
            if let Some(ret) = &self.return_type {
                format!("-> {ret}")
            } else {
                String::default()
            }
        )
    }
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<FunctionSignature<Dst>>
    for FunctionSignature<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> FunctionSignature<Dst> {
        FunctionSignature {
            parameters: self.parameters.cast_into(),
            return_type: self.return_type,
            src_ref: self.src_ref,
        }
    }
}

/// A function scope `{}`
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "Name: Serialize", deserialize = "Name: Deserialize<'de>"))]
pub struct Scope<Name: ir::NameSpec>(pub Refer<Box<[FunctionStatement<Name>]>>);

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<Scope<Dst>> for Scope<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> Scope<Dst> {
        Scope(self.0.cast_into())
    }
}

#[derive(Debug, Clone, Hash, From, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "Name: Serialize", deserialize = "Name: Deserialize<'de>"))]
pub enum FunctionExpression<Name: ir::NameSpec = ir::SymbolPath> {
    Invalid,
    Literal(ir::Literal),
    Name(Name),
    Scope(Scope<Name>),
    If(ir::If<FunctionExpression<Name>>),
    Call(ir::Call<FunctionExpression<Name>>),
}

impl<Name: ir::NameSpec> SingleIdentifier for FunctionExpression<Name> {
    fn single_identifier(&self) -> Option<&microcad_lang_base::Identifier> {
        match self {
            FunctionExpression::Name(name) => name.single_identifier(),
            _ => None,
        }
    }
}

impl<Name: ir::NameSpec> ir::ExprSpec for FunctionExpression<Name> {
    type Name = Name;
    type Body = Scope<Name>;
}

impl<Name: LowerName> CastInto<ir::FunctionExpression<Name>> for ir::ConstantExpression<Name> {
    fn cast_into(self: ir::ConstantExpression<Name>) -> ir::FunctionExpression<Name> {
        match self {
            ir::ConstantExpression::Invalid => ir::FunctionExpression::Invalid,
            ir::ConstantExpression::Literal(literal) => ir::FunctionExpression::Literal(literal),
            ir::ConstantExpression::Name(name) => ir::FunctionExpression::Name(name),
            ir::ConstantExpression::Call(call) => ir::FunctionExpression::Call(call.cast_into()),
        }
    }
}

impl<T: ir::NameSpec, Name: ir::NameSpec> CastInto<FunctionExpression<T>>
    for FunctionExpression<Name>
where
    Name: Into<T>,
{
    fn cast_into(self) -> FunctionExpression<T> {
        use FunctionExpression::*;
        match self {
            Invalid => Invalid,
            Literal(literal) => Literal(literal),
            Name(name) => Name(name.into()),
            Scope(scope) => Scope(scope.cast_into()),
            If(if_) => If(if_.cast_into()),
            Call(call) => Call(call.cast_into()),
        }
    }
}

impl<Name: ir::NameSpec> SrcReferrer for FunctionExpression<Name> {
    fn src_ref(&self) -> SrcRef {
        match &self {
            FunctionExpression::Invalid => SrcRef::none(),
            FunctionExpression::Literal(literal) => literal.src_ref(),
            FunctionExpression::Name(name) => name.src_ref(),
            FunctionExpression::Scope(scope) => scope.0.src_ref(),
            FunctionExpression::If(if_expr) => if_expr.src_ref,
            FunctionExpression::Call(call) => call.src_ref,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "Name: Serialize", deserialize = "Name: Deserialize<'de>"))]
pub struct ReturnStatement<Name: ir::NameSpec> {
    pub expr: Option<FunctionExpression<Name>>,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<ReturnStatement<Dst>> for ReturnStatement<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> ReturnStatement<Dst> {
        ReturnStatement {
            expr: self.expr.map(|value| value.cast_into()),
            keyword_src_ref: self.keyword_src_ref,
            src_ref: self.src_ref,
        }
    }
}

#[derive(Debug, Clone, derive_more::From, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "Name: Serialize", deserialize = "Name: Deserialize<'de>"))]
pub enum FunctionStatement<Name: ir::NameSpec> {
    /// `a = 42`
    Local(ir::LocalAssignment<FunctionExpression<Name>>),
    /// `{ a = 23; a }`
    Scope(ir::Scope<Name>),
    /// `print("Test");`
    Call(ir::Call<FunctionExpression<Name>>),
    /// `if { a } else { b }`
    If(ir::If<FunctionExpression<Name>>),
    /// Tail expression: `42`
    Tail(FunctionExpression<Name>),
    /// `return 42;`
    /// Possibly lowered from the tail expression of an `ast::StatementList`
    Return(ReturnStatement<Name>),
}

impl<Src: ir::NameSpec, Dst: ir::NameSpec> CastInto<FunctionStatement<Dst>>
    for FunctionStatement<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> FunctionStatement<Dst> {
        use FunctionStatement::*;
        match self {
            Local(local_assignment) => Local(local_assignment.cast_into()),
            Call(call) => Call(call.cast_into()),
            Scope(scope) => Scope(scope.cast_into()),
            If(if_) => If(if_.cast_into()),
            Tail(tail) => Tail(tail.cast_into()),
            Return(return_statement) => Return(return_statement.cast_into()),
        }
    }
}

impl<Name: ir::NameSpec> SrcReferrer for FunctionStatement<Name> {
    fn src_ref(&self) -> SrcRef {
        use FunctionStatement::*;
        match &self {
            Local(local_assignment) => local_assignment.src_ref,
            Call(call) => call.src_ref,
            Scope(scope) => scope.0.src_ref(),
            If(if_) => if_.src_ref,
            Tail(expr) => expr.src_ref(),
            Return(return_statement) => return_statement.src_ref,
        }
    }
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct FunctionItems {
    /// use ...
    pub aliases: ir::Aliases,
    /// const FOO =
    pub constants: Box<[ir::Constant]>,
}

impl IsDefault for FunctionItems {
    fn is_default(&self) -> bool {
        self.aliases.is_default() && self.constants.is_default()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    /// Source ref for the whole definition
    pub src_ref: SrcRef,
    /// Outer attributes
    pub outer_attr: ir::OuterAttributes,
    /// public / private
    pub visibility: ir::Visibility,
    /// SrcRef of the `fn` keyword
    pub keyword_ref: SrcRef,
    /// Name of the function
    pub id: ir::Identifier,
    /// Function signature
    pub signature: ir::FunctionSignature,
    /// #![...]
    pub inner_attr: ir::InnerAttributes,

    pub items: ir::FunctionItems,

    /// Function statements
    pub statements: Box<[ir::FunctionStatement<ir::SymbolPath>]>,
}
