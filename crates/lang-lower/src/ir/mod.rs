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

pub use microcad_lang_base::Identifier;
pub use microcad_lang_types::ty::{MatrixType, QuantityType, TupleType, Ty, Type, Unit};

use microcad_lang_base::{Refer, SrcRef};
use microcad_lang_proc_macros::SrcReferrer;

use crate::ir;

/// Visibility of an entity.
///
/// This is used to determine if an entity is public or private.
/// By default, entities are private.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Visibility {
    /// Private visibility
    #[default]
    Private,
    /// Public visibility
    Public,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Private => Ok(()),
            Visibility::Public => write!(f, "pub "),
        }
    }
}

/// Type within source code.
#[derive(Clone, Debug, PartialEq, SrcReferrer)]
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
#[derive(Debug)]
pub struct ExplicitAlias {
    pub attr: ir::Attributes,
    pub visibility: ir::Visibility,
    pub keyword_src_ref: SrcRef,
    pub path: QualifiedName,
    pub id: Identifier,
}

/// `use std::geo2d::*`
#[derive(Debug)]
pub struct WildcardAlias {
    pub attr: ir::Attributes,
    pub visibility: ir::Visibility,
    pub keyword_src_ref: SrcRef,
    pub path: QualifiedName,
}

/// Aliases lowered from `use` statements.
#[derive(Debug)]
pub struct Aliases {
    pub explicit_aliases: Box<[ExplicitAlias]>,
    pub wildcards: Box<[WildcardAlias]>,
}
