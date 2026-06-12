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

/// A function scope `{}`
#[derive(Debug, Serialize)]
#[serde(bound(serialize = "NAME: Serialize"))]
pub struct Scope<NAME: Serialize>(pub Refer<FunctionStatements<NAME>>);

/// Generic Access
type Access<ELEMENT, NAME> = ir::ElementAccess<FunctionExpression<NAME>, ELEMENT>;

/// A method call
type MethodCall<NAME> = Access<ir::Call<FunctionExpression<NAME>>, NAME>;

#[derive(Debug, Serialize)]
#[serde(bound(serialize = "NAME: Serialize"))]
pub enum FunctionExpression<NAME: Serialize = ir::QualifiedName> {
    Invalid,
    Literal(ir::Literal),
    Name(NAME),
    FormatString(ir::FormatString),
    ArrayExpression(ir::ArrayExpression<FunctionExpression<NAME>>),
    TupleExpression(ir::TupleExpression<FunctionExpression<NAME>>),
    Scope(Scope<NAME>),
    If(ir::If<FunctionExpression<NAME>, Scope<NAME>>),
    Call(ir::Call<FunctionExpression<NAME>>),
    BinaryOp(ir::BinaryOp<FunctionExpression<NAME>>),
    UnaryOp(ir::UnaryOp<FunctionExpression<NAME>>),
    /// Access an element of an array (`a[0]`)
    ArrayAccess(Access<Box<FunctionExpression<NAME>>, NAME>),
    TupleAccess(Access<ir::Identifier, NAME>),
    /// Call to a method: `[2,3].len()`
    MethodCall(MethodCall<NAME>),
}

impl<NAME: Serialize> ir::ExpressionKind for FunctionExpression<NAME> {
    type Name = NAME;
}

#[derive(Debug, Serialize)]
#[serde(bound(serialize = "NAME: Serialize"))]
pub struct ReturnStatement<NAME: Serialize> {
    pub value: Option<FunctionExpression<NAME>>,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
}

#[derive(Debug, derive_more::From, Serialize)]
#[serde(bound(serialize = "NAME: Serialize"))]
pub enum FunctionStatement<NAME: Serialize> {
    /// `a = 42`
    Local(ir::LocalAssignment<FunctionExpression<NAME>>),
    /// `{ a = 23; }`
    Expression(ir::FunctionExpression<NAME>),
    /// `return 42;`
    /// Possibly lowered from the tail expression of an `ast::StatementList`
    Return(ReturnStatement<NAME>),
}

#[derive(Debug, Serialize)]
#[serde(bound(serialize = "NAME: Serialize"))]
pub struct FunctionStatements<NAME: Serialize>(pub Box<[FunctionStatement<NAME>]>);

impl<NAME: Serialize> IsDefault for FunctionStatements<NAME> {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}

#[derive(Debug, Serialize)]
pub struct FunctionItems {
    /// use ...
    #[serde(skip_serializing_if = "is_default", default)]
    pub aliases: ir::Aliases,
    /// const FOO =
    #[serde(skip_serializing_if = "is_default", default)]
    pub constants: ir::Constants,
}

impl IsDefault for FunctionItems {
    fn is_default(&self) -> bool {
        self.aliases.is_default() && self.constants.is_default()
    }
}

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

    #[serde(skip_serializing_if = "is_default", default)]
    pub items: ir::FunctionItems,

    /// Function statements
    #[serde(skip_serializing_if = "is_default", default)]
    pub statements: ir::FunctionStatements<ir::QualifiedName>,
}

#[derive(Debug, Default, Serialize)]
pub struct Functions(pub Box<[Function]>);

impl IsDefault for Functions {
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}
