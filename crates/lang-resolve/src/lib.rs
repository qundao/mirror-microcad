// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve;

pub(crate) mod tree;

pub mod mir;
pub mod rst;

use microcad_lang_base::{CompilationResult, Source};
use microcad_lang_lower::Ir;

/// The mid-level intermediat
pub use mir::Mir;

pub use rst::Rst;

pub use resolve::{Resolve, ResolveContext, ResolveResult, scaffold};

pub fn resolve(_source: &Source, _mir: &Mir) -> CompilationResult<Rst> {
    todo!()
}

pub use scaffold::scaffold;
