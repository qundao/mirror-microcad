// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad intermediate representation (IR) elements.
//!
//! Every AST element in the µcad language is parsed into an intermediate representation in this module.

pub mod assignment;
pub mod attribute;
pub mod constant;
pub mod expression;
pub mod function;
pub mod module;
pub mod parameter;
pub mod source;
pub mod workbench;

pub use assignment::*;
pub use attribute::*;
pub use constant::*;
pub use expression::*;
pub use function::*;
pub use module::*;
pub use parameter::*;
pub use source::*;
pub use workbench::*;

pub use microcad_lang_base::{Identifier, element::Visibility};
pub use microcad_lang_types::ty::{MatrixType, QuantityType, TupleType, Ty, Type, Unit};

use microcad_lang_base::{IsDefault, Refer, SrcRef, is_default};
use microcad_lang_proc_macros::SrcReferrer;
use serde::{Deserialize, Serialize};

use crate::ir;

/// Type within source code.
#[derive(Clone, Debug, Hash, PartialEq, SrcReferrer, Serialize, Deserialize)]
pub struct TypeAnnotation(pub Refer<Type>);

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Ty for TypeAnnotation {
    fn ty(&self) -> Type {
        self.0.value.clone()
    }
}

/// `use std::geo2d::Circle as C` => (path = "std::geo2d::Circle", id = "C")
/// `use std::geo2d::Circle` => (path = "std::geo2d::Circle", id = "Circle")
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct ExplicitAlias {
    pub attr: ir::OuterAttributes,
    pub visibility: ir::Visibility,
    #[serde(skip_serializing_if = "SrcRef::is_none", default)]
    pub keyword_src_ref: SrcRef,
    pub path: QualifiedName,
    pub id: Identifier,
    pub src_ref: SrcRef,
}

/// `use std::geo2d::*`
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct WildcardAlias {
    #[serde(skip_serializing_if = "is_default", default)]
    pub attr: ir::OuterAttributes,
    pub visibility: ir::Visibility,
    #[serde(skip_serializing_if = "SrcRef::is_none", default)]
    pub keyword_src_ref: SrcRef,
    pub path: QualifiedName,
    pub src_ref: SrcRef,
}

/// Aliases lowered from `use` statements.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct Aliases {
    #[serde(skip_serializing_if = "is_default", default)]
    pub explicit_aliases: Box<[ExplicitAlias]>,
    #[serde(skip_serializing_if = "is_default", default)]
    pub wildcards: Box<[WildcardAlias]>,
}

impl IsDefault for Aliases {
    fn is_default(&self) -> bool {
        self.explicit_aliases.is_default() && self.wildcards.is_default()
    }
}
