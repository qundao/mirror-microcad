// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve_context;
mod resolve_error;

mod symbol;

use microcad_lang_base::{CompilationResult, Source};
use microcad_lang_lower::Ir;
pub use resolve_context::*;
pub use resolve_error::*;

pub use symbol::{Symbol, Symbols};

/// Trait to resolve an IR node into a symbol.
pub trait Resolve<T = Symbol> {
    fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<T>;
}

pub fn resolve(source: &Source, ir: &Ir) -> CompilationResult<Symbol> {
    todo!()
}
