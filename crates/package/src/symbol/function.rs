// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_lower::ir;
use microcad_lang_types::{FunctionType, Tuple};
use serde::{Deserialize, Serialize};

pub type FunctionExpression = ir::FunctionExpression;
pub type FunctionStatement = ir::FunctionStatement;

pub type ReturnStatement = ir::ReturnStatement;

pub use ir::Literal;
pub type Scope = ir::Scope;
pub type If = ir::If<FunctionExpression>;

/// A call to a function.
pub type Call = ir::Call<FunctionExpression>;

pub type Argument = ir::Argument<FunctionExpression>;
pub type ArgumentList = ir::ArgumentList<FunctionExpression>;

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    // pub attr: FunctionAttributes,
    pub ty: FunctionType,
    /// A tuple of default parameters
    pub default_parameters: Tuple, // TODO Evaluate to tuple from Vec<(Identifier, ConstantExpression)>,
    /// The function statements to be evaluated.
    pub statements: Box<[FunctionStatement]>,
}
