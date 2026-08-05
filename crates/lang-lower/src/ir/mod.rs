// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad intermediate representation (IR) elements.
//!
//! Every AST element in the µcad language is parsed into an intermediate representation in this module.

pub mod assignment;
pub mod attribute;
pub mod cast_into;
pub mod constant;
pub mod expression;
pub mod function;
pub mod module;
pub mod parameter;
pub mod source;
pub mod workbench;

pub use assignment::*;
pub use attribute::*;
pub use cast_into::*;
pub use constant::*;
pub use expression::*;
pub use function::*;
pub use module::*;
pub use parameter::*;
pub use source::*;
pub use workbench::*;

pub use microcad_lang_base::{Identifier, element::Visibility};
pub use microcad_lang_types::ty::{MatrixType, QuantityType, TupleType, Ty, Unit};

use derive_more::{Deref, Display};
use microcad_lang_base::SrcRef;
use serde::{Deserialize, Serialize};

use crate::ir;

#[derive(Debug, Default, Display, Deref, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[display("{}", ty)]
pub struct Type {
    #[deref]
    pub ty: microcad_lang_types::Type,
    pub src_ref: SrcRef,
}

/// `use std::geo2d::Circle as C` => (path = "std::geo2d::Circle", id = "C")
/// `use std::geo2d::Circle` => (path = "std::geo2d::Circle", id = "Circle")
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct ExplicitAlias {
    pub attr: ir::OuterAttributes,
    pub visibility: ir::Visibility,
    pub keyword_src_ref: SrcRef,
    pub path: SymbolPath,
    pub id: Identifier,
    pub src_ref: SrcRef,
}

/// `use std::geo2d::*`
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct WildcardAlias {
    pub attr: ir::OuterAttributes,
    pub visibility: ir::Visibility,
    pub keyword_src_ref: SrcRef,
    pub path: SymbolPath,
    pub src_ref: SrcRef,
}

/// Aliases lowered from `use` statements.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct Aliases {
    pub explicit_aliases: Box<[ExplicitAlias]>,
    pub wildcards: Box<[WildcardAlias]>,
}
