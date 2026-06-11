// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::{IsDefault, ir, is_default};

use microcad_lang_base::{Refer, SrcRef};
use serde::Serialize;

/// Parameters and return type of a function
#[derive(Debug, Serialize)]
pub struct FunctionSignature {
    /// Function's parameters
    pub parameters: ir::ParameterList,
    /// Function's return type
    pub return_type: Option<ir::TypeAnnotation>,
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

type Access<ELEMENT> = ir::ElementAccess<FunctionExpression, ELEMENT>;
type MethodCall = ir::Call<FunctionExpression>;

#[derive(Debug, Serialize)]
pub enum FunctionExpression {
    Invalid,
    Literal(ir::Literal),
    Name(ir::QualifiedName),
    FormatString(ir::FormatString),
    ArrayExpression(ir::ArrayExpression<FunctionExpression>),
    TupleExpression(ir::TupleExpression<FunctionExpression>),
    Scope(Scope),
    If(ir::If<FunctionExpression, Scope>),
    Call(ir::Call<FunctionExpression>),
    BinaryOp(ir::BinaryOp<FunctionExpression>),
    UnaryOp(ir::UnaryOp<FunctionExpression>),
    /// Access an element of an array (`a[0]`)
    ArrayAccess(Access<Box<FunctionExpression>>),
    TupleAccess(Access<ir::Identifier>),
    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to a value
    MethodCall(Access<MethodCall>),
}

#[derive(Debug, Serialize)]
pub struct ReturnStatement {
    pub value: Option<FunctionExpression>,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
}

#[derive(Debug, derive_more::From, Serialize)]
pub enum FunctionStatement {
    /// `a = 42`
    Local(ir::LocalAssignment<FunctionExpression>),
    /// `{ a = 23; }`
    Expression(ir::FunctionExpression),
    /// `return 42;`
    /// Possibly lowered from the tail expression of an `ast::StatementList`
    Return(ReturnStatement),
}

#[derive(Debug, Serialize)]
pub struct FunctionStatements(pub Box<[FunctionStatement]>);

impl IsDefault for FunctionStatements {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}

#[derive(Debug, Serialize)]
pub struct Scope(pub Refer<FunctionStatements>);

#[derive(Debug, Serialize)]
pub struct Function {
    /// Source ref for the whole definition
    pub src_ref: SrcRef,
    /// Outer attributes
    #[serde(skip_serializing_if = "is_default", default)]
    pub outer_attr: ir::OuterAttributes,
    /// public / private
    pub visibility: ir::Visibility,
    /// SrcRef of the `fn` keyword
    #[serde(skip_serializing_if = "SrcRef::is_none", default)]
    pub keyword_ref: SrcRef,
    /// Name of the function
    pub id: ir::Identifier,
    /// Function signature
    pub signature: ir::FunctionSignature,
    /// #![...]
    #[serde(skip_serializing_if = "is_default", default)]
    pub inner_attr: ir::InnerAttributes,
    /// use ...
    #[serde(skip_serializing_if = "is_default", default)]
    pub aliases: ir::Aliases,
    /// const FOO =
    #[serde(skip_serializing_if = "is_default", default)]
    pub constants: ir::Constants,
    /// Function statements
    #[serde(skip_serializing_if = "is_default", default)]
    pub statements: ir::FunctionStatements,
}

#[derive(Debug, Default, Serialize)]
pub struct Functions(pub Box<[Function]>);

impl IsDefault for Functions {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}
