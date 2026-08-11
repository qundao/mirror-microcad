// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::{CastInto, ir, lower::LowerPath};

use derive_more::From;
use microcad_lang_base::{SingleIdentifier, SrcRef, SrcReferrer};
use serde::{Deserialize, Serialize};

/// Parameters and return type of a function
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct FunctionSignature<Path: ir::PathSpec = ir::Path> {
    /// Function's parameters
    pub parameters: ir::ParameterList<Path>,
    /// Function's return type
    pub return_type: Option<ir::Type>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<Path: ir::PathSpec> std::fmt::Display for FunctionSignature<Path>
where
    Path: std::fmt::Display,
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

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<FunctionSignature<Dst>>
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
#[serde(bound(serialize = "Path: Serialize", deserialize = "Path: Deserialize<'de>"))]
pub struct Scope<Path: ir::PathSpec> {
    pub statements: Box<[FunctionStatement<Path>]>,
    pub src_ref: SrcRef,
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<Scope<Dst>> for Scope<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> Scope<Dst> {
        Scope {
            statements: self.statements.cast_into(),
            src_ref: self.src_ref,
        }
    }
}

#[derive(Debug, Clone, Hash, From, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "Path: Serialize", deserialize = "Path: Deserialize<'de>"))]
pub enum FunctionExpression<Path: ir::PathSpec = ir::Path> {
    Invalid,
    Literal(ir::Literal),
    Path(Path),
    Scope(Scope<Path>),
    If(ir::If<FunctionExpression<Path>>),
    Call(ir::Call<FunctionExpression<Path>>),
}

impl<Path: ir::PathSpec> SingleIdentifier for FunctionExpression<Path> {
    fn single_identifier(&self) -> Option<&microcad_lang_base::Identifier> {
        match self {
            FunctionExpression::Path(name) => name.single_identifier(),
            _ => None,
        }
    }
}

impl<Path: ir::PathSpec> ir::ExprSpec for FunctionExpression<Path> {
    type Path = Path;
    type Body = Scope<Path>;
}

impl<Path: LowerPath> CastInto<ir::FunctionExpression<Path>> for ir::ConstantExpression<Path> {
    fn cast_into(self: ir::ConstantExpression<Path>) -> ir::FunctionExpression<Path> {
        match self {
            ir::ConstantExpression::Invalid => ir::FunctionExpression::Invalid,
            ir::ConstantExpression::Literal(literal) => ir::FunctionExpression::Literal(literal),
            ir::ConstantExpression::Path(path) => ir::FunctionExpression::Path(path),
            ir::ConstantExpression::Call(call) => ir::FunctionExpression::Call(call.cast_into()),
        }
    }
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<FunctionExpression<Dst>>
    for FunctionExpression<Src>
where
    Src: Into<Dst>,
{
    fn cast_into(self) -> FunctionExpression<Dst> {
        use FunctionExpression::*;
        match self {
            Invalid => Invalid,
            Literal(literal) => Literal(literal),
            Path(name) => Path(name.into()),
            Scope(scope) => Scope(scope.cast_into()),
            If(if_) => If(if_.cast_into()),
            Call(call) => Call(call.cast_into()),
        }
    }
}

impl<Path: ir::PathSpec> SrcReferrer for FunctionExpression<Path> {
    fn src_ref(&self) -> SrcRef {
        match &self {
            FunctionExpression::Invalid => SrcRef::none(),
            FunctionExpression::Literal(literal) => literal.src_ref(),
            FunctionExpression::Path(name) => name.src_ref(),
            FunctionExpression::Scope(scope) => scope.src_ref,
            FunctionExpression::If(if_expr) => if_expr.src_ref,
            FunctionExpression::Call(call) => call.src_ref,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "Path: Serialize", deserialize = "Path: Deserialize<'de>"))]
pub struct ReturnStatement<Path: ir::PathSpec> {
    pub expr: Option<FunctionExpression<Path>>,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<ReturnStatement<Dst>> for ReturnStatement<Src>
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

#[derive(Debug, Clone, From, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "Path: Serialize", deserialize = "Path: Deserialize<'de>"))]
pub enum FunctionStatement<Path: ir::PathSpec> {
    /// `a = 42`
    Local(ir::LocalAssignment<FunctionExpression<Path>>),
    /// `{ a = 23; a }`
    Scope(ir::Scope<Path>),
    /// `print("Test");`
    Call(ir::Call<FunctionExpression<Path>>),
    /// `if { a } else { b }`
    If(ir::If<FunctionExpression<Path>>),
    /// Tail expression: `42`
    Tail(FunctionExpression<Path>),
    /// A return statement: `return 42;`
    Return(ReturnStatement<Path>),
}

impl<Src: ir::PathSpec, Dst: ir::PathSpec> CastInto<FunctionStatement<Dst>>
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

impl<Path: ir::PathSpec> SrcReferrer for FunctionStatement<Path> {
    fn src_ref(&self) -> SrcRef {
        use FunctionStatement::*;
        match &self {
            Local(local_assignment) => local_assignment.src_ref,
            Call(call) => call.src_ref,
            Scope(scope) => scope.src_ref,
            If(if_) => if_.src_ref,
            Tail(expr) => expr.src_ref(),
            Return(return_statement) => return_statement.src_ref,
        }
    }
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct FunctionItems<Path: ir::PathSpec = ir::Path> {
    /// use ...
    pub aliases: ir::Aliases<Path>,
    /// const FOO =
    pub constants: Box<[ir::Constant<Path>]>,
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function<Path: ir::PathSpec = ir::Path> {
    /// Source ref for the whole definition
    pub src_ref: SrcRef,
    /// Outer attributes
    pub outer_attr: ir::OuterAttributes<Path>,
    /// public / private
    pub visibility: ir::Visibility,
    /// SrcRef of the `fn` keyword
    pub keyword_ref: SrcRef,
    /// Name of the function
    pub id: ir::Identifier,
    /// Function signature
    pub signature: ir::FunctionSignature<Path>,
    /// #![...]
    pub inner_attr: ir::InnerAttributes<Path>,

    pub items: ir::FunctionItems<Path>,

    /// Function statements
    pub statements: Box<[ir::FunctionStatement<Path>]>,
}
