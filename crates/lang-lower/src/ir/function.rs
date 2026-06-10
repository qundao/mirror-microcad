// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::ir;

use microcad_lang_base::{Refer, SrcRef};

/// Parameters and return type of a function
#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Option<FunctionExpression>,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
}

#[derive(Debug, derive_more::From)]
pub enum FunctionStatement {
    Local(ir::LocalAssignment<FunctionExpression>),
    Scope(ir::Scope),
    If(ir::If<FunctionExpression, Scope>),
    Return(ReturnStatement),
}

#[derive(Debug)]
pub struct FunctionStatements(pub Box<[FunctionStatement]>);

#[derive(Debug)]
pub struct Scope(pub Refer<FunctionStatements>);

#[derive(Debug)]
pub struct Function {
    /// Source ref for the whole definition
    pub src_ref: SrcRef,
    /// Outer attributes
    pub outer_attr: ir::Attributes,
    /// public / private
    pub visibility: ir::Visibility,
    /// SrcRef of the `fn` keyword
    pub keyword_ref: SrcRef,
    /// Name of the function
    pub id: ir::Identifier,
    /// Function signature
    pub signature: ir::FunctionSignature,
    /// #![...]
    pub inner_attr: ir::Attributes,
    /// use ...
    pub aliases: ir::Aliases,
    /// const FOO =
    pub constants: ir::Constants,
    /// Function statements
    pub statements: ir::FunctionStatements,
}

#[derive(Debug)]
pub struct Functions(pub Box<[Function]>);
