// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::value::Value;
use microcad_lang_base::{Identifier, SrcRef};
pub use microcad_lang_lower::hir::{
    ArgumentList, ArrayExpression, BinaryOp, FormatString, If, TupleExpression, UnaryOp,
};

use crate::Symbol;

#[derive(Debug)]
pub enum Element<T> {
    Attribute(Identifier),
    /// Access property or a field of a tuple.
    Field(Identifier),
    /// A method to be called
    Method(Call<T>),
    /// Array Element
    ArrayElement(Box<T>),
}

#[derive(Debug)]
pub struct ElementAccess<T> {
    pub lhs: Box<T>,
    pub element: Element<T>,
    pub src_ref: SrcRef,
}

#[derive(Debug)]
pub struct Call<T> {
    /// Symbol to be called.
    pub symbol: Symbol,
    /// Argument list of the call.
    pub argument_list: ArgumentList<T>,
    /// Source code reference.
    pub src_ref: SrcRef,
}
