// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::{
    CastInto,
    ir::{self, ExprSpec},
};

use derive_more::From;
use microcad_lang_base::{SingleIdentifier, SrcRef, SrcReferrer};
use microcad_lang_types::{FunctionType, Tuple, Ty, Value};
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

/// Builder methods for testing.
impl FunctionSignature {
    pub fn new(parameters: impl Into<ir::ParameterList>) -> Self {
        Self {
            parameters: parameters.into(),
            return_type: None,
            src_ref: SrcRef::none(),
        }
    }

    pub fn with_return_type(mut self, ty: impl Into<ir::Type>) -> Self {
        self.return_type = Some(ty.into());
        self
    }
}

/// Retrieval methods for evaluation
impl FunctionSignature {
    pub fn ty(&self) -> FunctionType {
        FunctionType {
            parameters: Some(microcad_lang_types::FunctionTypeParameters(
                self.parameters
                    .parameters
                    .iter()
                    .map(|param| {
                        match param
                            .default_value
                            .as_ref()
                            .and_then(|expr| expr.value().cloned())
                        {
                            Some(value) => (param.id.clone(), value.ty()),
                            None => (param.id.clone(), param.ty.ty.clone()),
                        }
                    })
                    .collect(),
            )),
            return_ty: self.return_type.as_ref().map(|ty| Box::new(ty.ty.clone())),
        }
    }

    pub fn default_parameter_values(&self) -> Tuple {
        self.parameters.default_values()
    }
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
    Constant(ir::ConstantValue),
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

    fn value(&self) -> Option<&Value> {
        match &self {
            FunctionExpression::Constant(constant) => Some(constant.value()),
            _ => None,
        }
    }
}

impl CastInto<ir::FunctionExpression> for ir::ConstantExpression {
    fn cast_into(self: ir::ConstantExpression) -> ir::FunctionExpression {
        match self {
            ir::ConstantExpression::Invalid => ir::FunctionExpression::Invalid,
            ir::ConstantExpression::Constant(literal) => ir::FunctionExpression::Constant(literal),
            ir::ConstantExpression::Path(path) => ir::FunctionExpression::Path(path),
            ir::ConstantExpression::Call(call) => ir::FunctionExpression::Call(call.cast_into()),
        }
    }
}

impl SrcReferrer for FunctionExpression {
    fn src_ref(&self) -> SrcRef {
        match &self {
            FunctionExpression::Invalid => SrcRef::none(),
            FunctionExpression::Constant(literal) => literal.src_ref(),
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
    /// Attributes
    pub attr: ir::Attributes,
    /// Function signature.
    pub signature: ir::FunctionSignature,
    /// Function statements
    pub statements: Box<[ir::FunctionStatement]>,
}

impl Function {
    pub fn ty(&self) -> FunctionType {
        self.signature.ty()
    }

    pub fn default_parameters(&self) -> Tuple {
        self.signature.default_parameter_values()
    }
}
