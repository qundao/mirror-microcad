// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::{CastInto, ir};

use derive_more::From;
use microcad_lang_base::{SingleIdentifier, SrcRef, SrcReferrer};
use serde::{Deserialize, Serialize};

/// Parameters and return type of a function
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct FunctionSignature {
    /// Function's parameters
    pub parameters: ir::ParameterList,
    /// Function's return type
    pub return_type: Option<ir::Type>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl std::fmt::Display for FunctionSignature {
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

/// A function scope `{}`
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    pub statements: Box<[FunctionStatement]>,
    pub src_ref: SrcRef,
}

#[non_exhaustive]
#[derive(Debug, Clone, Hash, From, PartialEq, Serialize, Deserialize)]
pub enum FunctionExpression {
    Invalid,
    Literal(ir::Literal),
    Path(ir::Path),
    Scope(Scope),
    If(ir::If<FunctionExpression>),
    Call(ir::Call<FunctionExpression>),
}

impl SingleIdentifier for FunctionExpression {
    fn single_identifier(&self) -> Option<&microcad_lang_base::Identifier> {
        match self {
            FunctionExpression::Path(name) => name.single_identifier(),
            _ => None,
        }
    }
}

impl ir::ExprSpec for FunctionExpression {
    type Body = Scope;
}

impl CastInto<ir::FunctionExpression> for ir::ConstantExpression {
    fn cast_into(self: ir::ConstantExpression) -> ir::FunctionExpression {
        match self {
            ir::ConstantExpression::Invalid => ir::FunctionExpression::Invalid,
            ir::ConstantExpression::Literal(literal) => ir::FunctionExpression::Literal(literal),
            ir::ConstantExpression::Path(path) => ir::FunctionExpression::Path(path),
            ir::ConstantExpression::Call(call) => ir::FunctionExpression::Call(call.cast_into()),
        }
    }
}

impl SrcReferrer for FunctionExpression {
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
pub struct ReturnStatement {
    pub expr: Option<FunctionExpression>,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
}

#[non_exhaustive]
#[derive(Debug, Clone, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum FunctionStatement {
    /// `a = 42`
    Local(ir::LocalAssignment<FunctionExpression>),
    /// `{ a = 23; a }`
    Scope(ir::Scope),
    /// `print("Test");`
    Call(ir::Call<FunctionExpression>),
    /// `if { a } else { b }`
    If(ir::If<FunctionExpression>),
    /// Tail expression: `42`
    Tail(FunctionExpression),
    /// A return statement: `return 42;`
    Return(ReturnStatement),
}

impl SrcReferrer for FunctionStatement {
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

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub signature: ir::FunctionSignature,
    /// Function statements
    pub statements: Box<[ir::FunctionStatement]>,
}
