// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve;

mod error;

mod source_unit;

pub mod locate;

pub use source_unit::SourceUnit;

pub use error::{ResolveError, ResolveResult};

pub use resolve::{ResolveContext, resolve};

pub mod library;
pub mod symbol;

pub use library::{Manifest, ManifestError};

pub use symbol::{
    SymbolDef, SymbolId, SymbolNode, SymbolNodeExt, SymbolNodeMut, SymbolNodeRef, SymbolTree,
};

#[macro_export]
macro_rules! argument_list {
    ( $( $key:ident = $val:expr ),* $(,)? ) => {
        $crate::symbol::ArgumentList::from_iter(
        [
            $(
                $crate::symbol::Argument::named(stringify!($key), $val)
            ),*
        ])
    };
}
