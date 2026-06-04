// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement syntax element.

use crate::ir;

use microcad_lang_base::{Identifier, SrcRef, SrcReferrer};

/// Use declaration.
///
/// A use declaration is an element of a use statement.
/// It can be a single symbol, all symbols from a module, or an alias.
///
/// ```mcad
/// use std::print;
/// use std::*;
/// use std::print as p;
/// ```
///
#[derive(Clone, Debug)]
pub enum UseDeclaration {
    /// Import symbols given as qualified names: `use a, b`
    Use(ir::QualifiedName),
    /// Import all symbols from a module: `use std::*`
    UseAll(ir::QualifiedName),
    /// Import as alias: `use a as b`
    UseAs(ir::QualifiedName, Identifier),
}

impl SrcReferrer for UseDeclaration {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(.., name) => name.src_ref(),
            Self::UseAll(.., name) => name.src_ref(),
            Self::UseAs(.., name, _) => name.src_ref(),
        }
    }
}

impl std::fmt::Display for UseDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UseDeclaration::Use(name) => write!(f, "{name}"),
            UseDeclaration::UseAll(name) => write!(f, "{name}::*"),
            UseDeclaration::UseAs(name, alias) => {
                write!(f, "{name} as {alias}")
            }
        }
    }
}
