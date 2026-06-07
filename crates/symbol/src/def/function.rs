// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::def::expression::{
    ArrayExpression, BinaryOp, Call, ElementAccess, FormatString, If, TupleExpression, UnaryOp,
};

use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_types::{Type, Value};

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Option<FunctionExpression>,
    pub keyword_src_ref: SrcRef,
    pub src_ref: SrcRef,
}

#[derive(Debug, derive_more::From)]
pub enum FunctionStatement {
    Local(Identifier, FunctionExpression),
    Expression(FunctionExpression),
    Return(ReturnStatement),
}

#[derive(Debug, derive_more::From)]
pub enum FunctionExpression {
    Value(Value),
    FormatString(FormatString),
    ArrayExpression(ArrayExpression<FunctionExpression>),
    TupleExpression(TupleExpression<FunctionExpression>),
    Body(Vec<FunctionStatement>),
    If(If<FunctionExpression, Vec<FunctionStatement>>),
    Call(Call<FunctionExpression>),
    BinaryOp(BinaryOp<FunctionExpression>),
    UnaryOp(UnaryOp<FunctionExpression>),
    ElementAccess(ElementAccess<FunctionExpression>),
}

#[derive(Debug)]
pub struct Function {
    parameters: ParameterValueList,
    return_type: Option<Type>,
    script: Vec<FunctionStatement>,
}
