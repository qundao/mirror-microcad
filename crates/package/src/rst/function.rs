// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_lower::ir;
use microcad_lang_types::Type;
use serde::{Deserialize, Serialize};

use crate::rst::{ParameterList, ResolvedName};

pub type FunctionExpression = ir::FunctionExpression<ResolvedName>;
pub type FunctionStatement = ir::FunctionStatement<ResolvedName>;

pub use ir::Literal;
pub type FormatString = ir::FormatString<ResolvedName>;
pub type ArrayExpression = ir::ArrayExpression<FunctionExpression>;
pub type ArrayExpressionInner = ir::ArrayExpressionInner<FunctionExpression>;
pub type ListExpression = ir::ListExpression<FunctionExpression>;
pub type RangeExpression = ir::RangeExpression<FunctionExpression>;
pub type RangeFirst = ir::RangeFirst<FunctionExpression>;
pub type RangeLast = ir::RangeLast<FunctionExpression>;

pub type TupleExpression = ir::TupleExpression<FunctionExpression>;
pub type Scope = ir::Scope<ResolvedName>;
pub type If = ir::If<FunctionExpression>;
pub type Call = ir::Call<FunctionExpression>;
pub type BinaryOp = ir::BinaryOp<FunctionExpression>;
pub type UnaryOp = ir::UnaryOp<FunctionExpression>;
pub type ArrayAccess = ir::ElementAccess<FunctionExpression, ResolvedName>;
pub type TupleAccess = ir::ElementAccess<ir::Identifier, ResolvedName>;
pub type MethodCall = ir::ElementAccess<ir::Call<FunctionExpression>, ResolvedName>;

pub type ArgumentList = ir::ArgumentList<FunctionExpression>;
pub type UnnamedArgument = ir::UnnamedArgument<FunctionExpression>;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    // pub attr: FunctionAttributes,
    pub parameters: ParameterList,
    pub return_ty: Option<Type>,
    pub statements: Box<[FunctionStatement]>,
}
