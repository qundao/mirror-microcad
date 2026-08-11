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
pub mod path;
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
pub use path::{Path, PathSpec, UnresolvedPath};
pub use source::*;
pub use workbench::*;

pub use microcad_lang_base::{Identifier, element::Visibility};
pub use microcad_lang_types::ty::{MatrixType, QuantityType, TupleType, Ty, Unit};

use derive_more::{Deref, Display};
use microcad_lang_base::{SrcRef, SymbolName, Unresolve, Unresolver};
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
pub struct ExplicitAlias<Path: ir::PathSpec = ir::Path> {
    pub attr: ir::OuterAttributes<Path>,
    pub visibility: ir::Visibility,
    pub keyword_src_ref: SrcRef,
    pub path: Path,
    pub id: Identifier,
    pub src_ref: SrcRef,
}

impl Unresolve<ir::OuterAttributes<SymbolName>> for ir::OuterAttributes {
    fn unresolve_symbols<U: microcad_lang_base::Unresolver>(
        self,
        unresolver: &mut U,
    ) -> ir::OuterAttributes<SymbolName> {
        todo!()
    }
}

impl Unresolve<ExplicitAlias<SymbolName>> for ExplicitAlias {
    fn unresolve_symbols<U: Unresolver>(self, unresolver: &mut U) -> ExplicitAlias<SymbolName> {
        ExplicitAlias {
            attr: self.attr.unresolve_symbols(unresolver),
            visibility: self.visibility,
            keyword_src_ref: self.keyword_src_ref,
            path: unresolver.unresolve(self.path),
            id: self.id,
            src_ref: self.src_ref,
        }
    }
}

/// `use std::geo2d::*`
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct WildcardAlias<Path: ir::PathSpec = ir::Path> {
    pub attr: ir::OuterAttributes<Path>,
    pub visibility: ir::Visibility,
    pub keyword_src_ref: SrcRef,
    pub path: Path,
    pub src_ref: SrcRef,
}

impl Unresolve<WildcardAlias<SymbolName>> for WildcardAlias {
    fn unresolve_symbols<U: Unresolver>(self, unresolver: &mut U) -> WildcardAlias<SymbolName> {
        WildcardAlias {
            attr: self.attr.unresolve_symbols(unresolver),
            visibility: self.visibility,
            keyword_src_ref: self.keyword_src_ref,
            path: self.path.unresolve_symbols(unresolver),
            src_ref: self.src_ref,
        }
    }
}

/// Aliases lowered from `use` statements.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct Aliases<Path: ir::PathSpec = ir::Path> {
    pub explicit_aliases: Box<[ExplicitAlias<Path>]>,
    pub wildcards: Box<[WildcardAlias<Path>]>,
}
