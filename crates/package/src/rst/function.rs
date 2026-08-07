// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_lower::ir;
use microcad_lang_types::{FunctionType, Tuple};
use serde::{Deserialize, Serialize};

use crate::rst::ResolvedName;

pub type FunctionExpression = ir::FunctionExpression<ResolvedName>;
pub type FunctionStatement = ir::FunctionStatement<ResolvedName>;

pub type ReturnStatement = ir::ReturnStatement<ResolvedName>;

pub use ir::Literal;
pub type Scope = ir::Scope<ResolvedName>;
pub type If = ir::If<FunctionExpression>;

/// A call to a function.
pub type Call = ir::Call<FunctionExpression>;

pub type Argument = ir::Argument<FunctionExpression>;
pub type ArgumentList = ir::ArgumentList<FunctionExpression>;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    // pub attr: FunctionAttributes,
    pub ty: FunctionType,
    /// A tuple of default parameters
    pub default_parameters: Tuple,
    /// The function statements to be evaluated.
    pub statements: Box<[FunctionStatement]>,
}
